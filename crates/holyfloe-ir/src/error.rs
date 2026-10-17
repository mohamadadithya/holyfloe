use thiserror::Error;

/// Printed by the CLI as `error[<code>]: <message>`, then the hint.
#[derive(Error, Clone, Debug, PartialEq, Eq)]
#[error("{message}")]
pub struct IrError {
    pub code: &'static str,
    pub message: String,
    pub tag: Option<String>,
    /// CEM module path.
    pub module: Option<String>,
    pub hint: String,
}

impl IrError {
    pub(crate) fn new(
        code: &'static str,
        message: impl Into<String>,
        tag: Option<&str>,
        module: Option<&str>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            tag: tag.map(str::to_owned),
            module: module.map(str::to_owned),
            hint: hint.into(),
        }
    }
}
