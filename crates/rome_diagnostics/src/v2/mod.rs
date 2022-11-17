pub mod adapters;
pub mod advice;
pub mod context;
pub mod diagnostic;
pub mod display;
pub mod error;
pub mod location;
pub mod serde;
pub mod panic;

#[doc(hidden)]
// Convenience re-export for procedural macro
pub use rome_console as console;

// Re-export macros from utility crates
pub use rome_diagnostics_categories::{category, category_concat, Category};
pub use rome_diagnostics_macros::Diagnostic;

pub use advice::{
    Advices, CodeFrameAdvice, CommandAdvice, DiffAdvice, LogAdvice, LogCategory, Visit,
};
pub use context::{Context, DiagnosticExt};
pub use diagnostic::{Diagnostic, DiagnosticTags, Severity};
pub use display::{
    set_bottom_frame, Backtrace, MessageAndDescription, PrintDescription, PrintDiagnostic,
};
pub use error::{Error, Result};
pub use location::{FileId, FilePath, LineIndex, LineIndexBuf, Location, Resource, SourceCode};

pub mod prelude {
    //! Anonymously re-exports all the traits declared by this module, this is
    //! intended to be imported as `use rome_diagnostics::v2::prelude::*;` to
    //! automatically bring all these traits into the ambient context

    pub use super::advice::{Advices as _, Visit as _};
    pub use super::context::{Context as _, DiagnosticExt as _};
    pub use super::diagnostic::Diagnostic as _;
}
