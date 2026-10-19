//! The generate pipeline on disk, shared by the `holyfloe` binary and `holyfloe-napi`: read the
//! manifest (or extract one from Lit source), config and dependency manifests, build the IR,
//! emit, write through `holyfloe-fs`.

use std::fs;
use std::path::{Path, PathBuf};

use holyfloe_emit_react::{ReactConfig, ReactMode};
use holyfloe_ir::{Config, Dependency, PackageInfo};

#[cfg(feature = "watch")]
mod watch;
#[cfg(feature = "watch")]
pub use watch::{Watcher, watch};

#[derive(Clone)]
pub struct Options {
    pub input: Input,
    pub out: PathBuf,
    pub frameworks: Vec<Framework>,
    /// `holyfloe.config.json`.
    pub config: Option<PathBuf>,
    /// Overrides `react.mode` from the config.
    pub react_mode: Option<ReactMode>,
}

/// Where components come from. The `package.json` nearest to it names the package and its exports.
#[derive(Clone, Debug)]
pub enum Input {
    /// A `custom-elements.json`, e.g. from the CEM analyzer.
    Cem(PathBuf),
    /// A directory of Lit sources, read with `holyfloe-lit` (D-089).
    Lit(PathBuf),
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framework {
    React,
    Vue,
    Svelte,
}

impl Framework {
    /// Output subdirectory under `--out`.
    pub fn dir(self) -> &'static str {
        match self {
            Self::React => "react",
            Self::Vue => "vue",
            Self::Svelte => "svelte",
        }
    }
}

/// One framework's output.
#[derive(Debug)]
pub struct Run {
    pub framework: Framework,
    pub report: holyfloe_fs::Report,
}

/// Displays as the CLI prints it: `error[E0xx]: message`, then the hint on the next line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub code: Option<String>,
    pub message: String,
    pub hint: Option<String>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            Some(code) => write!(f, "error[{code}]: {}", self.message)?,
            None => write!(f, "error: {}", self.message)?,
        }
        if let Some(hint) = &self.hint {
            write!(f, "\n  hint: {hint}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl Error {
    fn at(path: &Path, e: impl std::fmt::Display) -> Self {
        Self::plain(format!("{}: {e}", path.display()))
    }

    fn plain(message: String) -> Self {
        Self {
            code: None,
            message,
            hint: None,
        }
    }
}

pub fn generate(opts: &Options) -> Result<Vec<Run>, Error> {
    let (cem, pkg, pkg_dir) = match &opts.input {
        Input::Cem(path) => {
            let cem = holyfloe_cem::parse(&read(path)?).map_err(|e| Error::at(path, e))?;
            let (pkg, dir) = nearest_package(path)?;
            (cem, pkg, dir)
        }
        // The `package.json` start path makes a package.json in `dir` itself count.
        Input::Lit(dir) => {
            let (pkg, pkg_dir) = nearest_package(&dir.join("package.json"))?;
            (lit_manifest(dir, &pkg_dir, &opts.out)?, pkg, pkg_dir)
        }
    };
    let deps = dependencies(&cem, &pkg_dir)?;
    let raw: serde_json::Value = match &opts.config {
        Some(path) => serde_json::from_str(&read(path)?).map_err(|e| Error::at(path, e))?,
        None => serde_json::json!({}),
    };
    let config_err =
        |e: serde_json::Error| Error::at(opts.config.as_deref().unwrap_or(Path::new("config")), e);
    let cfg: Config = serde_json::from_value(raw.clone()).map_err(config_err)?;
    let mut react: ReactConfig = match raw.get("react") {
        Some(v) => serde_json::from_value(v.clone()).map_err(config_err)?,
        None => ReactConfig::default(),
    };
    if let Some(mode) = opts.react_mode {
        react.mode = mode;
    }

    let lib = holyfloe_ir::build(&cem, &pkg, &deps, &cfg).map_err(|e| {
        let at = match e.module {
            Some(m) if !e.message.contains(&m) => format!(" ({m})"),
            _ => String::new(),
        };
        Error {
            code: Some(e.code.to_owned()),
            message: format!("{}{at}", e.message),
            hint: Some(e.hint),
        }
    })?;
    // Emit every framework before writing any, so an emitter error leaves no partial output.
    let mut emitted = Vec::new();
    for framework in [Framework::React, Framework::Vue, Framework::Svelte] {
        if !opts.frameworks.contains(&framework) {
            continue;
        }
        let files = match framework {
            Framework::React => holyfloe_emit_react::emit(&lib, &react),
            Framework::Vue => holyfloe_emit_vue::emit(&lib),
            Framework::Svelte => holyfloe_emit_svelte::emit(&lib),
        };
        let mut files = files.map_err(|e| Error {
            code: e.code().map(str::to_owned),
            message: e.to_string(),
            hint: e.hint(),
        })?;
        for f in &mut files {
            f.path = format!("{}/{}", framework.dir(), f.path);
        }
        emitted.push((framework, files));
    }
    emitted
        .into_iter()
        .map(|(framework, files)| {
            let report = holyfloe_fs::write_if_changed(&opts.out, &files)
                .map_err(|e| Error::plain(e.to_string()))?;
            Ok(Run { framework, report })
        })
        .collect()
}

#[cfg(feature = "lit")]
fn lit_manifest(dir: &Path, root: &Path, out: &Path) -> Result<holyfloe_cem::Package, Error> {
    let manifest = holyfloe_lit::extract(root, &lit_sources(dir, out)?)
        .map_err(|e| Error::plain(e.to_string()))?;
    holyfloe_cem::parse(&manifest.to_string()).map_err(|e| Error::at(dir, e))
}

#[cfg(not(feature = "lit"))]
fn lit_manifest(dir: &Path, _: &Path, _: &Path) -> Result<holyfloe_cem::Package, Error> {
    Err(Error::at(
        dir,
        "this build of holyfloe has no Lit input; pass a manifest",
    ))
}

/// Source files under `dir`: `.ts`, `.tsx`, `.mts`, `.js`, `.mjs`, `.jsx`, not declarations,
/// `node_modules`, dot-directories or `out` (the wrappers holyfloe writes are not sources).
pub fn lit_sources(dir: &Path, out: &Path) -> Result<Vec<PathBuf>, Error> {
    let out = real(out);
    let mut files = Vec::new();
    let mut dirs = vec![dir.to_owned()];
    while let Some(d) = dirs.pop() {
        for entry in fs::read_dir(&d).map_err(|e| Error::at(&d, e))? {
            let path = entry.map_err(|e| Error::at(&d, e))?.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if path.is_dir() {
                if !(name.starts_with('.') || name == "node_modules" || real(&path) == out) {
                    dirs.push(path);
                }
            } else if is_source(&path) {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

pub fn is_source(path: &Path) -> bool {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let ext = path.extension().unwrap_or_default();
    ["ts", "tsx", "mts", "js", "mjs", "jsx"]
        .iter()
        .any(|x| ext == *x)
        && !name.ends_with(".d.ts")
        && !name.ends_with(".d.mts")
}

/// `path` with symlinks resolved, also when it does not exist yet (an `--out` not yet written).
pub fn real(path: &Path) -> PathBuf {
    let abs = std::path::absolute(path).unwrap_or_else(|_| path.to_owned());
    match (fs::canonicalize(&abs), abs.parent(), abs.file_name()) {
        (Ok(p), _, _) => p,
        (Err(_), Some(parent), Some(name)) => real(parent).join(name),
        _ => abs,
    }
}

fn read(path: &Path) -> Result<String, Error> {
    fs::read_to_string(path).map_err(|e| Error::at(path, e))
}

/// Manifests of the packages that superclasses and mixins come from (D-069), found like Node
/// does: `node_modules/<name>` from the package directory upward, each dependency's own
/// dependencies from its real directory. Packages without a `customElements` manifest (`lit`)
/// are skipped.
fn dependencies(cem: &holyfloe_cem::Package, dir: &Path) -> Result<Vec<Dependency>, Error> {
    let mut deps: Vec<Dependency> = Vec::new();
    let mut queue: Vec<(String, PathBuf)> = holyfloe_ir::referenced_packages(cem)
        .into_iter()
        .map(|name| (name, dir.to_owned()))
        .collect();
    while let Some((name, from)) = queue.pop() {
        if deps.iter().any(|d| d.name == name) {
            continue;
        }
        let Some(root) = from
            .ancestors()
            .map(|d| d.join("node_modules").join(&name))
            .find(|root| root.join("package.json").is_file())
        else {
            continue;
        };
        let pkg_path = root.join("package.json");
        let value: serde_json::Value =
            serde_json::from_str(&read(&pkg_path)?).map_err(|e| Error::at(&pkg_path, e))?;
        let Some(manifest) = value["customElements"].as_str() else {
            continue;
        };
        let manifest = root.join(manifest);
        let dep_cem =
            holyfloe_cem::parse(&read(&manifest)?).map_err(|e| Error::at(&manifest, e))?;
        let real = fs::canonicalize(&root).map_err(|e| Error::at(&root, e))?;
        queue.extend(
            holyfloe_ir::referenced_packages(&dep_cem)
                .into_iter()
                .map(|n| (n, real.clone())),
        );
        deps.push(Dependency { name, cem: dep_cem });
    }
    Ok(deps)
}

/// The nearest `package.json` above `start` that has a `name`, and its directory. Nameless
/// ones are skipped: they are build markers such as `dist/esm/package.json` = `{"type": "module"}` (tshy).
fn nearest_package(start: &Path) -> Result<(PackageInfo, PathBuf), Error> {
    let abs = std::path::absolute(start).map_err(|e| Error::at(start, e))?;
    for path in abs.ancestors().skip(1).map(|dir| dir.join("package.json")) {
        if !path.is_file() {
            continue;
        }
        let value: serde_json::Value =
            serde_json::from_str(&read(&path)?).map_err(|e| Error::at(&path, e))?;
        if value.get("name").is_some() {
            let dir = path.parent().unwrap_or(Path::new(".")).to_owned();
            return Ok((
                serde_json::from_value(value).map_err(|e| Error::at(&path, e))?,
                dir,
            ));
        }
    }
    Err(Error {
        code: None,
        message: format!(
            "no package.json with a `name` found above {}",
            start.display()
        ),
        hint: Some("the manifest or sources must live inside the package they describe".to_owned()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lit_sources_skip_out_dependencies_hidden_dirs_and_declarations() {
        let root = std::env::temp_dir().join(format!("holyfloe-sources-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        for f in [
            "src/a.ts",
            "src/sub/b.js",
            "src/c.d.ts",
            "src/gen/react/X.tsx",
            "src/node_modules/dep/d.ts",
            "src/.cache/e.ts",
            "src/readme.md",
        ] {
            let p = root.join(f);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, "").unwrap();
        }
        let files = lit_sources(&root.join("src"), &root.join("src/gen")).unwrap();
        let rel: Vec<_> = files
            .iter()
            .map(|f| {
                f.strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        assert_eq!(rel, ["src/a.ts", "src/sub/b.js"]);
        fs::remove_dir_all(&root).unwrap();
    }
}
