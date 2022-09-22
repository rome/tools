#![deny(rust_2018_idioms)]

use serde::{Deserialize, Serialize};

pub mod file;
pub mod v2;

mod diagnostic;
mod emit;
mod suggestion;

pub use diagnostic::{Diagnostic, Footer, SubDiagnostic};
pub use emit::Emitter;
pub use file::Span;
pub use suggestion::*;

pub use rome_console::codespan::Severity;
pub use termcolor;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum DiagnosticTag {
    Unnecessary,
    Deprecated,
    Both,
}

impl DiagnosticTag {
    pub fn is_unnecessary(&self) -> bool {
        matches!(self, DiagnosticTag::Unnecessary | DiagnosticTag::Both)
    }

    pub fn is_deprecated(&self) -> bool {
        matches!(self, DiagnosticTag::Deprecated | DiagnosticTag::Both)
    }
}

/// Indicates how a tool should manage this suggestion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Applicability {
    /// The suggestion is definitely what the user intended.
    /// This suggestion should be automatically applied.
    Always,
    /// The suggestion may be what the user intended, but it is uncertain.
    /// The suggestion should result in valid JavaScript/TypeScript code if it is applied.
    MaybeIncorrect,
    /// The suggestion contains placeholders like `(...)` or `{ /* fields */ }`.
    /// The suggestion cannot be applied automatically because it will not result in valid JavaScript/TypeScript code.
    /// The user will need to fill in the placeholders.
    HasPlaceholders,
    /// The applicability of the suggestion is unknown.
    Unspecified,
}

pub const MAXIMUM_DISPLAYABLE_DIAGNOSTICS: u16 = 200;
