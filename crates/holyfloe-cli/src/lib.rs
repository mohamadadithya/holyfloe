//! The generate pipeline on disk, shared by the `holyfloe` binary and `holyfloe-napi`: read the
//! manifest, config and dependency manifests, build the IR, emit, write through `holyfloe-fs`.

use std::fs;
use std::path::{Path, PathBuf};

use holyfloe_emit_react::{ReactConfig, ReactMode};
use holyfloe_ir::{Config, Dependency, PackageInfo};

pub struct Options {
    /// `custom-elements.json`. The `package.json` nearest to it names the package and its exports.
    pub cem: PathBuf,
    pub out: PathBuf,
    pub frameworks: Vec<Framework>,
    /// `holyfloe.config.json`.
    pub config: Option<PathBuf>,
    /// Overrides `react.mode` from the config.
    pub react_mode: Option<ReactMode>,
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
    let cem = holyfloe_cem::parse(&read(&opts.cem)?).map_err(|e| Error::at(&opts.cem, e))?;
    let (pkg, pkg_dir) = nearest_package(&opts.cem)?;
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
    let mut runs = Vec::new();
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
        let report = holyfloe_fs::write_if_changed(&opts.out, &files)
            .map_err(|e| Error::plain(e.to_string()))?;
        runs.push(Run { framework, report });
    }
    Ok(runs)
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

/// The nearest `package.json` above the manifest that has a `name`, and its directory. Nameless
/// ones are skipped: they are build markers such as `dist/esm/package.json` = `{"type": "module"}` (tshy).
fn nearest_package(cem: &Path) -> Result<(PackageInfo, PathBuf), Error> {
    let abs = std::path::absolute(cem).map_err(|e| Error::at(cem, e))?;
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
            cem.display()
        ),
        hint: Some("the manifest must live inside the package it describes".to_owned()),
    })
}
