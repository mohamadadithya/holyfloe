//! holyfloe command-line interface.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};
use holyfloe_cli::{Framework, Input, Options};
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
    #[command(flatten)]
    source: Source,
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
    /// Keep running and regenerate when the input (manifest or Lit sources) or config changes.
    #[arg(long)]
    watch: bool,
}

/// Exactly one input. The `package.json` nearest to it names the package and its exports.
#[derive(Args)]
#[group(required = true, multiple = false)]
struct Source {
    /// `custom-elements.json`, e.g. from the CEM analyzer.
    #[arg(long)]
    cem: Option<PathBuf>,
    /// Directory of Lit sources to read instead of a manifest (no analyzer needed). Scanned
    /// recursively, skipping tests, stories, `node_modules`, dot-directories and `--out`.
    #[arg(long)]
    lit: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Copy)]
enum ReactModeArg {
    Wrapper,
    Types,
}

fn main() -> ExitCode {
    let Cmd::Generate(args) = Cli::parse().cmd;
    let opts = Options {
        // The group makes exactly one of the two present.
        input: match (args.source.cem, args.source.lit) {
            (Some(cem), _) => Input::Cem(cem),
            (None, dir) => Input::Lit(dir.unwrap_or_default()),
        },
        out: args.out,
        frameworks: args.frameworks,
        config: args.config,
        react_mode: args.react_mode.map(|m| match m {
            ReactModeArg::Wrapper => ReactMode::Wrapper,
            ReactModeArg::Types => ReactMode::Types,
        }),
    };
    if args.watch {
        return watch(opts);
    }
    match holyfloe_cli::generate(&opts) {
        Ok(runs) => {
            print_runs(&opts.out, &runs);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

/// Runs until killed. A failed run is printed and the watch goes on.
fn watch(opts: Options) -> ExitCode {
    let out = opts.out.clone();
    let report = move |res: Result<Vec<holyfloe_cli::Run>, holyfloe_cli::Error>| match res {
        Ok(runs) => print_runs(&out, &runs),
        Err(e) => eprintln!("{e}"),
    };
    let _watcher = match holyfloe_cli::watch(opts.clone(), report.clone()) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    report(holyfloe_cli::generate(&opts));
    println!("watching for changes (Ctrl-C to stop)");
    loop {
        std::thread::park();
    }
}

fn print_runs(out: &std::path::Path, runs: &[holyfloe_cli::Run]) {
    for run in runs {
        let dir = run.framework.dir();
        println!(
            "{dir}: {} written, {} unchanged in {}",
            run.report.written.len(),
            run.report.unchanged.len(),
            out.join(dir).display()
        );
    }
}
