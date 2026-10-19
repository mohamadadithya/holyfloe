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
    if o.frameworks.is_empty() {
        return Err(invalid(
            "frameworks is empty: list react, vue or svelte".to_owned(),
        ));
    }
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

#[cfg(feature = "watch")]
type Callback = ThreadsafeFunction<Vec<FrameworkReport>, (), Vec<FrameworkReport>, Code>;

/// What a watch shares between its JS handle and its event handler.
#[cfg(feature = "watch")]
struct Shared {
    callback: std::sync::Mutex<Option<Callback>>,
    watcher: std::sync::Mutex<Option<holyfloe_cli::Watcher>>,
}

/// Returned by `watch()`. Keeps the process alive until `close()`, also when it is not referenced.
#[napi]
pub struct Watcher(#[cfg(feature = "watch")] std::sync::Arc<Shared>);

#[napi]
impl Watcher {
    /// Stops watching. No callback runs after this returns.
    #[napi]
    pub fn close(&mut self) -> Result<(), Error> {
        #[cfg(feature = "watch")]
        {
            let callback = self.0.callback.lock().map(|mut c| c.take());
            if let Ok(Some(callback)) = callback {
                // Not a plain drop: that still delivers queued runs, abort discards them.
                #[allow(deprecated)]
                callback
                    .abort()
                    .map_err(|e| Error::new(e.status.into(), e.reason))?;
            }
            let watcher = self.0.watcher.lock().map(|mut w| w.take());
            drop(watcher);
        }
        Ok(())
    }
}

/// Generates now and again after each change to the manifest or config. The callback gets
/// `(error, reports)`; a failed run does not stop the watch. Throws in the wasm build (D-087).
#[napi]
// The full type, not `Callback`: napi derives the `.d.ts` callback signature from it.
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
        let shared = std::sync::Arc::new(Shared {
            callback: std::sync::Mutex::new(Some(callback)),
            watcher: std::sync::Mutex::new(None),
        });
        // The handler owns `shared`, and so the native watcher: a garbage-collected JS Watcher
        // must not stop the watch (as with `fs.watch`). Only `close()` breaks the cycle.
        let state = shared.clone();
        let send = move |res: Result<Vec<Run>, holyfloe_cli::Error>| {
            if let Ok(callback) = state.callback.lock()
                && let Some(callback) = callback.as_ref()
            {
                callback.call(
                    res.map(reports).map_err(to_js),
                    napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
                );
            }
        };
        let watcher = holyfloe_cli::watch(opts.clone(), send.clone()).map_err(to_js)?;
        if let Ok(mut slot) = shared.watcher.lock() {
            *slot = Some(watcher);
        }
        send(holyfloe_cli::generate(&opts));
        Ok(Watcher(shared))
    }
}
