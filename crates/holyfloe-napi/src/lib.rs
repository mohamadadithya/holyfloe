//! Node bindings: `generate()`, `watch()`, `version()`. Errors carry the holyfloe code (`E002`)
//! as `error.code`, and the hint on the message's second line.

use std::path::PathBuf;

use clap::ValueEnum;
use holyfloe_cli::{Framework, Options, Run};
use napi::Status;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

#[napi(object)]
pub struct GenerateOptions {
    /// `custom-elements.json`. The `package.json` nearest to it names the package and its exports.
    pub cem: String,
    /// Wrappers go to `<out>/<framework>/`.
    pub out: String,
    #[napi(ts_type = "Array<'react' | 'vue' | 'svelte'>")]
    pub frameworks: Vec<String>,
    /// `holyfloe.config.json`.
    pub config: Option<String>,
    /// Overrides `react.mode` from the config.
    #[napi(ts_type = "'wrapper' | 'types'")]
    pub react_mode: Option<String>,
}

#[napi(object)]
pub struct FrameworkReport {
    #[napi(ts_type = "'react' | 'vue' | 'svelte'")]
    pub framework: String,
    /// Paths relative to `out`, written because their content changed.
    pub written: Vec<String>,
    /// Paths relative to `out`, left untouched.
    pub unchanged: Vec<String>,
}

/// `error.code`: `E0xx` for holyfloe errors, `GenericFailure` for I/O and parse errors.
pub struct Code(String);

impl AsRef<str> for Code {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Status> for Code {
    fn from(s: Status) -> Self {
        Self(s.to_string())
    }
}

type Error = napi::Error<Code>;

fn to_js(e: holyfloe_cli::Error) -> Error {
    let code = e.code.unwrap_or_else(|| Status::GenericFailure.to_string());
    let mut msg = e.message;
    if let Some(hint) = e.hint {
        msg.push_str("\n  hint: ");
        msg.push_str(&hint);
    }
    Error::new(Code(code), msg)
}

fn invalid(msg: String) -> Error {
    Error::new(Status::InvalidArg.into(), msg)
}

fn options(o: GenerateOptions) -> Result<Options, Error> {
    let frameworks = o
        .frameworks
        .iter()
        .map(|f| {
            Framework::from_str(f, false)
                .map_err(|_| invalid(format!("unknown framework `{f}`: use react, vue or svelte")))
        })
        .collect::<Result<_, _>>()?;
    let react_mode = o
        .react_mode
        .map(|m| {
            serde_json::from_value(serde_json::Value::String(m.clone()))
                .map_err(|_| invalid(format!("unknown reactMode `{m}`: use wrapper or types")))
        })
        .transpose()?;
    Ok(Options {
        cem: PathBuf::from(o.cem),
        out: PathBuf::from(o.out),
        frameworks,
        config: o.config.map(PathBuf::from),
        react_mode,
    })
}

fn reports(runs: Vec<Run>) -> Vec<FrameworkReport> {
    runs.into_iter()
        .map(|r| FrameworkReport {
            framework: r.framework.dir().to_owned(),
            written: r.report.written,
            unchanged: r.report.unchanged,
        })
        .collect()
}

/// Generates wrappers, writing only files whose content changed.
#[napi]
pub fn generate(options: GenerateOptions) -> Result<Vec<FrameworkReport>, Error> {
    let opts = self::options(options)?;
    holyfloe_cli::generate(&opts).map(reports).map_err(to_js)
}

/// The holyfloe version.
#[napi]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returned by `watch()`. Keeps the process alive until `close()`.
#[napi]
pub struct Watcher(#[cfg(feature = "watch")] Option<holyfloe_cli::Watcher>);

#[napi]
impl Watcher {
    #[napi]
    pub fn close(&mut self) {
        #[cfg(feature = "watch")]
        {
            self.0 = None;
        }
    }
}

/// Generates now and again after each change to the manifest or config. The callback gets
/// `(error, reports)`; a failed run does not stop the watch. Throws in the wasm build (D-087).
#[napi]
pub fn watch(
    options: GenerateOptions,
    callback: ThreadsafeFunction<Vec<FrameworkReport>, (), Vec<FrameworkReport>, Code>,
) -> Result<Watcher, Error> {
    let opts = self::options(options)?;
    #[cfg(not(feature = "watch"))]
    {
        let _ = (opts, callback);
        Err(Error::new(
            Status::GenericFailure.into(),
            "watch() is not available in the wasm build of @holyfloe/core".to_owned(),
        ))
    }
    #[cfg(feature = "watch")]
    {
        let callback = std::sync::Arc::new(callback);
        let send = move |res: Result<Vec<Run>, holyfloe_cli::Error>| {
            callback.call(
                res.map(reports).map_err(to_js),
                napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
            );
        };
        let watcher = holyfloe_cli::watch(opts.clone(), send.clone()).map_err(to_js)?;
        send(holyfloe_cli::generate(&opts));
        Ok(Watcher(Some(watcher)))
    }
}
