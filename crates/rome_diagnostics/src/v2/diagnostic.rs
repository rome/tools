use std::{fmt::Debug, io};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use rome_console::fmt;

use super::{Category, Location, Visitor};

/// The `Diagnostic` trait
pub trait Diagnostic: Debug {
    /// The category of a diagnostic uniquely identifying this
    /// diagnostic type, such as `lint/correctness/noArguments`, `args/invalid`
    /// or `format/disabled`
    fn category(&self) -> Option<&Category> {
        None
    }

    /// The severity defines whether this diagnostic reports an error, a
    /// warning, an information or a hint to the user
    fn severity(&self) -> Severity {
        Severity::Error
    }

    /// The description is a text-only explanation of the issue this diagnostic
    /// is reporting, intended for display contexts that do not support rich
    /// markup such as in-editor popovers
    ///
    /// The description should generally be as exhaustive as possible, since
    /// the clients that do not support rendering markup will not render the
    /// advices for the diagnostic either
    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = fmt;
        Ok(())
    }

    /// An explanation of the issue this diagnostic is reporting
    ///
    /// In general it's better to keep this message as short as possible, and
    /// instead rely on advices to better convey contextual explanations to the
    /// user
    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let _ = fmt;
        Ok(())
    }

    /// Advices are the main building blocks used compose rich errors. They are
    /// implemented using a visitor pattern, where consumers of a diagnostic
    /// can visit the object and collect the advices that make it up for the
    /// purpose of display or introspection
    fn advices(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        let _ = visitor;
        Ok(())
    }

    /// Diagnostics can defines additional advices to be printed if the user
    /// requires more detail about the diagnostic
    fn verbose_advices(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        let _ = visitor;
        Ok(())
    }

    /// A diagnostic can be tied to a specific "location": this can be a file,
    /// memory buffer, command line argument, etc. It may also be tied to a
    /// specific text range within the content of that location. Finally, it
    /// may also provide the source string for that location (this is required
    /// in order to display a code frame advice for the diagnostic)
    fn location(&self) -> Option<Location<'_>> {
        None
    }

    /// Tags convey additional boolean metadata about the nature of a diagnostic:
    /// - If the diagnostic can be automatically fixed
    /// - If the diagnostic resulted from and internal error
    /// - If the diagnostic is being emitted as part of a crash / fatal error
    /// - If the diagnostic is a warning about a piece of unused or unnecessary code
    /// - If the diagnostic is a warning about a piece of deprecated or obsolete code
    fn tags(&self) -> DiagnosticTags {
        DiagnosticTags::empty()
    }

    /// Similarly to the `source` method of the [std::error::Error] trait, this
    /// returns another diagnostic that's the logical "cause" for this issue.
    /// For instance, a "request failed" diagnostic may have been cause by a
    /// "deserialization error". This allows low-level error to be wrapped in
    /// higher level concepts, while retaining enough information to display
    /// and fix the underlying issue
    fn source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
/// The severity to associate to a diagnostic
pub enum Severity {
    /// Reports an error
    Error,
    /// Reports a warning
    Warning,
    /// Reports an information
    Information,
    /// Reports a hint
    Hint,
}

/// Internal enum used to automatically generate bit offsets for [DiagnosticTags]
/// and help with the implementation of `serde` and `schemars` for tags
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub(super) enum DiagnosticTag {
    Fixable,
    Internal,
    Fatal,
    Unnecessary,
    Deprecated,
}

bitflags! {
    pub struct DiagnosticTags: u8 {
        /// This diagnostic has a fix suggestion
        const FIXABLE = 1 << DiagnosticTag::Fixable as u8;
        /// This diagnostic results from an internal error
        const INTERNAL = 1 << DiagnosticTag::Internal as u8;
        /// This diagnostic results from a crash
        const FATAL = 1 << DiagnosticTag::Fatal as u8;
        /// This diagnostic tags unused or unnecessary code
        const UNNECESSARY = 1 << DiagnosticTag::Unnecessary as u8;
        /// This diagnostic tags deprecated or obsolete code
        const DEPRECATED = 1 << DiagnosticTag::Deprecated as u8;
    }
}
