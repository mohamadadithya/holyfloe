//! holyfloe command-line interface.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};
use holyfloe_emit_react::{ReactConfig, ReactMode};
use holyfloe_ir::{Config, PackageInfo};

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
    let pkg_path = nearest_package_json(&args.cem)?;
    let pkg: PackageInfo = serde_json::from_str(&read(&pkg_path)?)
        .map_err(|e| format!("error: {}: {e}", pkg_path.display()))?;
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

    let lib = holyfloe_ir::build(&cem, &pkg, &cfg).map_err(|e| {
        let at = match e.module {
            Some(m) if !e.message.contains(&m) => format!(" ({m})"),
            _ => String::new(),
        };
        coded(Some(e.code), &format!("{}{at}", e.message), Some(&e.hint))
    })?;
    if args.frameworks.contains(&Framework::React) {
        let mut files = holyfloe_emit_react::emit(&lib, &react)
            .map_err(|e| coded(e.code(), &e.to_string(), e.hint().as_deref()))?;
        for f in &mut files {
            f.path = format!("react/{}", f.path);
        }
        let report = holyfloe_fs::write_if_changed(&args.out, &files)
            .map_err(|e| format!("error: {}: {e}", args.out.display()))?;
        println!(
            "react: {} written, {} unchanged in {}",
            report.written.len(),
            report.unchanged.len(),
            args.out.join("react").display()
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

fn nearest_package_json(cem: &Path) -> Result<PathBuf, String> {
    let abs = std::path::absolute(cem).map_err(|e| format!("error: {}: {e}", cem.display()))?;
    abs.ancestors()
        .skip(1)
        .map(|dir| dir.join("package.json"))
        .find(|p| p.is_file())
        .ok_or_else(|| {
            format!(
                "error: no package.json found above {}\n  hint: the manifest must live inside the package it describes",
                cem.display()
            )
        })
}
