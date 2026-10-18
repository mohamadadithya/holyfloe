//! Svelte 5 runes emitter. See `docs/emit-svelte.md`.

pub use holyfloe_emit_core::EmitError;
use holyfloe_emit_core::OutputFile;
use holyfloe_ir::Library;

pub fn emit(_lib: &Library) -> Result<Vec<OutputFile>, EmitError> {
    Ok(Vec::new())
}
