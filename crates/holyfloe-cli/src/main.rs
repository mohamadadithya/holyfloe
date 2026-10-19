//! holyfloe command-line interface.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};
use holyfloe_emit_react::{ReactConfig, ReactMode};
use holyfloe_ir::{Config, Dependency, PackageInfo};

#[derive(Parser)]
#[command(
    name = "holyfloe",
    version,
    about = "Typed framework wrappers for Lit web components"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Generate wrappers from a Custom Elements Manifest into `<out>/<framework>/`.
    Generate(Generate),
}

#[derive(Args)]
struct Generate {
    /// `custom-elements.json`. The `package.json` nearest to it names the package and its exports.
    #[arg(long)]
    cem: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long, value_delimiter = ',', required = true)]
    frameworks: Vec<Framework>,
    /// `holyfloe.config.json`.
    #[arg(long)]
    config: Option<PathBuf>,
    /// Overrides `react.mode` from the config.
    #[arg(long)]
    react_mode: Option<ReactModeArg>,
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq)]
enum Framework {
    React,
    Vue,
    Svelte,
}

#[derive(ValueEnum, Clone, Copy)]
enum ReactModeArg {
    Wrapper,
    Types,
}

fn main() -> ExitCode {
    let Cmd::Generate(args) = Cli::parse().cmd;
    match generate(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("{msg}");
            ExitCode::FAILURE
        }
    }
}

/// `Err` is the full message to print.
fn generate(args: &Generate) -> Result<(), String> {
    let cem = holyfloe_cem::parse(&read(&args.cem)?)
        .map_err(|e| format!("error: {}: {e}", args.cem.display()))?;
    let (pkg, pkg_dir) = nearest_package(&args.cem)?;
    let deps = dependencies(&cem, &pkg_dir)?;
    let raw: serde_json::Value = match &args.config {
        Some(path) => serde_json::from_str(&read(path)?)
            .map_err(|e| format!("error: {}: {e}", path.display()))?,
        None => serde_json::json!({}),
    };
    let config_err = |e: serde_json::Error| {
        let path = args.config.as_deref().unwrap_or(Path::new("config"));
        format!("error: {}: {e}", path.display())
    };
    let cfg: Config = serde_json::from_value(raw.clone()).map_err(config_err)?;
    let mut react: ReactConfig = match raw.get("react") {
        Some(v) => serde_json::from_value(v.clone()).map_err(config_err)?,
        None => ReactConfig::default(),
    };
    if let Some(mode) = args.react_mode {
        react.mode = match mode {
            ReactModeArg::Wrapper => ReactMode::Wrapper,
            ReactModeArg::Types => ReactMode::Types,
        };
    }

    let lib = holyfloe_ir::build(&cem, &pkg, &deps, &cfg).map_err(|e| {
        let at = match e.module {
            Some(m) if !e.message.contains(&m) => format!(" ({m})"),
            _ => String::new(),
        };
        coded(Some(e.code), &format!("{}{at}", e.message), Some(&e.hint))
    })?;
    for framework in [Framework::React, Framework::Vue, Framework::Svelte] {
        if !args.frameworks.contains(&framework) {
            continue;
        }
        let (dir, files) = match framework {
            Framework::React => ("react", holyfloe_emit_react::emit(&lib, &react)),
            Framework::Vue => ("vue", holyfloe_emit_vue::emit(&lib)),
            Framework::Svelte => ("svelte", holyfloe_emit_svelte::emit(&lib)),
        };
        let mut files = files.map_err(|e| coded(e.code(), &e.to_string(), e.hint().as_deref()))?;
        for f in &mut files {
            f.path = format!("{dir}/{}", f.path);
        }
        let report =
            holyfloe_fs::write_if_changed(&args.out, &files).map_err(|e| format!("error: {e}"))?;
        println!(
            "{dir}: {} written, {} unchanged in {}",
            report.written.len(),
            report.unchanged.len(),
            args.out.join(dir).display()
        );
    }
    Ok(())
}

/// `error[E0xx]: message`, then the hint on the next line.
fn coded(code: Option<&str>, message: &str, hint: Option<&str>) -> String {
    let mut out = match code {
        Some(code) => format!("error[{code}]: {message}"),
        None => format!("error: {message}"),
    };
    if let Some(hint) = hint {
        out.push_str("\n  hint: ");
        out.push_str(hint);
    }
    out
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("error: {}: {e}", path.display()))
}

/// Manifests of the packages that superclasses and mixins come from (D-069), found like Node
/// does: `node_modules/<name>` from the package directory upward, each dependency's own
/// dependencies from its real directory. Packages without a `customElements` manifest (`lit`)
/// are skipped.
fn dependencies(cem: &holyfloe_cem::Package, dir: &Path) -> Result<Vec<Dependency>, String> {
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
        let value: serde_json::Value = serde_json::from_str(&read(&pkg_path)?)
            .map_err(|e| format!("error: {}: {e}", pkg_path.display()))?;
        let Some(manifest) = value["customElements"].as_str() else {
            continue;
        };
        let manifest = root.join(manifest);
        let dep_cem = holyfloe_cem::parse(&read(&manifest)?)
            .map_err(|e| format!("error: {}: {e}", manifest.display()))?;
        let real =
            fs::canonicalize(&root).map_err(|e| format!("error: {}: {e}", root.display()))?;
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
fn nearest_package(cem: &Path) -> Result<(PackageInfo, PathBuf), String> {
    let abs = std::path::absolute(cem).map_err(|e| format!("error: {}: {e}", cem.display()))?;
    for path in abs.ancestors().skip(1).map(|dir| dir.join("package.json")) {
        if !path.is_file() {
            continue;
        }
        let parse_err = |e: serde_json::Error| format!("error: {}: {e}", path.display());
        let value: serde_json::Value = serde_json::from_str(&read(&path)?).map_err(parse_err)?;
        if value.get("name").is_some() {
            let dir = path.parent().unwrap_or(Path::new(".")).to_owned();
            return Ok((serde_json::from_value(value).map_err(parse_err)?, dir));
        }
    }
    Err(format!(
        "error: no package.json with a `name` found above {}\n  hint: the manifest must live inside the package it describes",
        cem.display()
    ))
}
