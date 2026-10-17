//! The six normalization passes, run by [`crate::build`] in this order (`docs/ir.md`).

pub(crate) mod classify_props;
pub(crate) mod drop_non_public;
pub(crate) mod flatten_inheritance;
pub(crate) mod infer_bindings;
pub(crate) mod name_derive;
pub(crate) mod validate;
