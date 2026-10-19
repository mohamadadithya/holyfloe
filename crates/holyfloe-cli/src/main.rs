//! holyfloe command-line interface.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};
use holyfloe_cli::{Framework, Options};
use holyfloe_emit_react::ReactMode;

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

#[derive(ValueEnum, Clone, Copy)]
enum ReactModeArg {
    Wrapper,
    Types,
}

fn main() -> ExitCode {
    let Cmd::Generate(args) = Cli::parse().cmd;
    let opts = Options {
        cem: args.cem,
        out: args.out,
        frameworks: args.frameworks,
        config: args.config,
        react_mode: args.react_mode.map(|m| match m {
            ReactModeArg::Wrapper => ReactMode::Wrapper,
            ReactModeArg::Types => ReactMode::Types,
        }),
    };
    match holyfloe_cli::generate(&opts) {
        Ok(runs) => {
            print_runs(&opts, &runs);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn print_runs(opts: &Options, runs: &[holyfloe_cli::Run]) {
    for run in runs {
        let dir = run.framework.dir();
        println!(
            "{dir}: {} written, {} unchanged in {}",
            run.report.written.len(),
            run.report.unchanged.len(),
            opts.out.join(dir).display()
        );
    }
}
