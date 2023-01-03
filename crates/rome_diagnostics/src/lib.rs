#![deny(rust_2018_idioms)]

use ::serde::{Deserialize, Serialize};

pub mod adapters;
pub mod advice;
pub mod context;
pub mod diagnostic;
pub mod display;
pub mod error;
pub mod location;
pub mod panic;
pub mod serde;

mod suggestion;

pub use self::suggestion::{Applicability, CodeSuggestion, FileSpan};
pub use termcolor;

#[doc(hidden)]
// Convenience re-export for procedural macro
pub use rome_console as console;

// Re-export macros from utility crates
pub use rome_diagnostics_categories::{category, category_concat, Category};
pub use rome_diagnostics_macros::Diagnostic;

pub use crate::advice::{
    Advices, CodeFrameAdvice, CommandAdvice, DiffAdvice, LogAdvice, LogCategory, Visit,
};
pub use crate::context::{Context, DiagnosticExt};
pub use crate::diagnostic::{Diagnostic, DiagnosticTags, Severity};
pub use crate::display::{
    set_bottom_frame, Backtrace, MessageAndDescription, PrintDescription, PrintDiagnostic,
};
pub use crate::error::{Error, Result};
pub use crate::location::{FilePath, LineIndex, LineIndexBuf, Location, Resource, SourceCode};
use rome_console::fmt::{Formatter, Termcolor};
use rome_console::markup;
use std::fmt::Write;

pub mod prelude {
    //! Anonymously re-exports all the traits declared by this module, this is
    //! intended to be imported as `use rome_diagnostics::prelude::*;` to
    //! automatically bring all these traits into the ambient context

    pub use crate::advice::{Advices as _, Visit as _};
    pub use crate::context::{Context as _, DiagnosticExt as _};
    pub use crate::diagnostic::Diagnostic as _;
}

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

pub const MAXIMUM_DISPLAYABLE_DIAGNOSTICS: u16 = 200;

/// Utility function for testing purpose. The function will print an [Error]
/// to a string, which is then returned by the function.
pub fn print_diagnostic_to_string(diagnostic: &Error) -> String {
    let mut buffer = termcolor::Buffer::no_color();

    Formatter::new(&mut Termcolor(&mut buffer))
        .write_markup(markup! {
            {PrintDiagnostic::verbose(diagnostic)}
        })
        .expect("failed to emit diagnostic");

    let mut content = String::new();
    writeln!(
        content,
        "{}",
        std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
    )
    .unwrap();

    content
}
