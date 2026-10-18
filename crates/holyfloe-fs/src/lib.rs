//! Output planning, content hashing, write-if-changed.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Component, Path, PathBuf};

use holyfloe_emit_core::OutputFile;

/// What `write_if_changed` did, by `OutputFile.path`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    pub written: Vec<String>,
    pub unchanged: Vec<String>,
}

/// Writes each file under `root`, skipping files whose bytes already match so their mtime is
/// kept (watchers and bundlers see no change). Every path is checked before anything is written:
/// absolute paths and `..` segments are `InvalidInput`, because emitters do not validate them.
pub fn write_if_changed(root: &Path, files: &[OutputFile]) -> io::Result<Report> {
    let targets = files
        .iter()
        .map(|f| target(root, &f.path))
        .collect::<io::Result<Vec<_>>>()?;
    let mut report = Report::default();
    for (f, path) in files.iter().zip(targets) {
        if fs::read(&path).is_ok_and(|old| old == f.contents.as_bytes()) {
            report.unchanged.push(f.path.clone());
            continue;
        }
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(&path, &f.contents)?;
        report.written.push(f.path.clone());
    }
    Ok(report)
}

fn target(root: &Path, rel: &str) -> io::Result<PathBuf> {
    let mut path = root.to_path_buf();
    for part in Path::new(rel).components() {
        match part {
            Component::Normal(p) => path.push(p),
            Component::CurDir => {}
            _ => {
                return Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("output path `{rel}` must be relative, without `..`"),
                ));
            }
        }
    }
    if path == root {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            format!("output path `{rel}` is empty"),
        ));
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::*;

    fn file(path: &str, contents: &str) -> OutputFile {
        OutputFile {
            path: path.into(),
            contents: contents.into(),
        }
    }

    fn temp_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("holyfloe-fs-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        root
    }

    #[test]
    fn writes_then_skips_unchanged() {
        let root = temp_root("skip");
        let files = [file("react/A.tsx", "a"), file("react/index.ts", "i")];
        let r = write_if_changed(&root, &files).unwrap();
        assert_eq!(r.written, ["react/A.tsx", "react/index.ts"]);
        assert_eq!(fs::read_to_string(root.join("react/A.tsx")).unwrap(), "a");

        let old = SystemTime::now() - Duration::from_secs(3600);
        let a = fs::File::options()
            .write(true)
            .open(root.join("react/A.tsx"))
            .unwrap();
        a.set_modified(old).unwrap();
        let r = write_if_changed(
            &root,
            &[file("react/A.tsx", "a"), file("react/index.ts", "j")],
        )
        .unwrap();
        assert_eq!(r.unchanged, ["react/A.tsx"]);
        assert_eq!(r.written, ["react/index.ts"]);
        let mtime = fs::metadata(root.join("react/A.tsx"))
            .unwrap()
            .modified()
            .unwrap();
        assert_eq!(mtime, old, "unchanged file must not be touched");
        assert_eq!(
            fs::read_to_string(root.join("react/index.ts")).unwrap(),
            "j"
        );
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn rejects_escaping_paths_before_writing() {
        let root = temp_root("reject");
        for bad in ["../x.ts", "react/../../x.ts", "/tmp/x.ts", "", "."] {
            let err = write_if_changed(&root, &[file("ok.ts", "ok"), file(bad, "x")]).unwrap_err();
            assert_eq!(err.kind(), ErrorKind::InvalidInput, "{bad:?}");
        }
        assert!(!root.exists(), "nothing is written when any path is bad");
    }
}
