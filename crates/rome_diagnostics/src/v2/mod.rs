pub mod adapters;
pub mod advice;
pub mod context;
pub mod diagnostic;
pub mod display;
pub mod error;
pub mod location;
pub mod serde;

use std::panic::UnwindSafe;

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

#[derive(Default, Debug)]
pub struct PanicError {
    pub info: String,
    pub backtrace: Option<std::backtrace::Backtrace>,
}

thread_local! {
    static LAST_PANIC: std::cell::Cell<Option<PanicError>> = std::cell::Cell::new(None);
}

impl std::fmt::Display for PanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = f.write_fmt(format_args!("{}\n", self.info));
        if let Some(backtrace) = &self.backtrace {
            f.write_fmt(format_args!("Backtrace: {}", backtrace))
        } else {
            r
        }
    }
}

/// Take and set a specific panic hook before calling `f` inside a `catch_unwind`, then
/// return the old set_hook.
///
/// If `f` panicks am `Error` with the panic message plus backtrace will be returned.
pub fn catch_unwind<F, R>(f: F) -> Result<R, PanicError>
where
    F: FnOnce() -> R + UnwindSafe,
{
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| {
        let info = info.to_string();
        let backtrace = std::backtrace::Backtrace::capture();
        LAST_PANIC.with(|cell| {
            cell.set(Some(PanicError {
                info,
                backtrace: Some(backtrace),
            }))
        })
    }));

    let result = std::panic::catch_unwind(f)
        .map_err(|_| LAST_PANIC.with(|cell| cell.take()).unwrap_or_default());

    std::panic::set_hook(prev);

    result
}

pub mod prelude {
    //! Anonymously re-exports all the traits declared by this module, this is
    //! intended to be imported as `use rome_diagnostics::v2::prelude::*;` to
    //! automatically bring all these traits into the ambient context

    pub use super::advice::{Advices as _, Visit as _};
    pub use super::context::{Context as _, DiagnosticExt as _};
    pub use super::diagnostic::Diagnostic as _;
}
