use std::io;

use rome_console::fmt::{self, Display};
use serde::{Deserialize, Serialize};

use super::{
    display::Backtrace,
    location::{AsPath, AsSourceCode, AsSpan},
    Location,
};

/// Trait implemented by types that support emitting advices into a diagnostic
pub trait IntoAdvices {
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()>;
}

/// The `Visitor` trait is used to collect advices from a diagnostic: a visitor
/// instance is provided to the [Diagnostic::advices](super::Diagnostic::advices)
/// and [Diagnostic::verbose_advices](super::Diagnostic::verbose_advices) methods,
/// and the diagnostic implementation is expected to call into the various `visit_*`
/// methods to communicate advices to the user
pub trait Visitor {
    /// Prints a single log entry with the provided category and markup
    fn visit_log(&mut self, category: LogCategory, text: &dyn fmt::Display) -> io::Result<()> {
        let _ = (category, text);
        Ok(())
    }

    /// Prints an unordered list of items
    fn visit_list(&mut self, list: &[&dyn fmt::Display]) -> io::Result<()> {
        let _ = list;
        Ok(())
    }

    /// Prints a code frame outlining the provided source location
    fn visit_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        let _ = location;
        Ok(())
    }

    /// Prints the diff between the `prev` and `next` strings
    fn visit_diff(&mut self, prev: &str, next: &str) -> io::Result<()> {
        let _ = (prev, next);
        Ok(())
    }

    /// Prints a Rust backtrace
    fn visit_backtrace(
        &mut self,
        title: &dyn fmt::Display,
        backtrace: &Backtrace,
    ) -> io::Result<()> {
        let _ = (title, backtrace);
        Ok(())
    }

    /// Prints a command to the user
    fn visit_command(&mut self, command: &str) -> io::Result<()> {
        let _ = command;
        Ok(())
    }

    /// Prints a group of advices under a common title
    fn visit_group(
        &mut self,
        title: &dyn fmt::Display,
        advice: &dyn IntoAdvices,
    ) -> io::Result<()> {
        let _ = (title, advice);
        Ok(())
    }
}

/// The category for a log advice, defines how the message should be presented
/// to the user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum LogCategory {
    /// The advice doesn't have any specific category, the message will be
    /// printed as plain markup
    None,
    /// Print the advices with the information style
    Info,
    /// Print the advices with the warning style
    Warn,
    /// Print the advices with the error style
    Error,
}

/// Utility type implementing [IntoAdvices] that emits a single log advice with
/// the provided category and text
#[derive(Debug)]
pub struct LogAdvice<T> {
    pub category: LogCategory,
    pub text: T,
}

impl<T: Display> IntoAdvices for LogAdvice<T> {
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        visitor.visit_log(self.category, &self.text)
    }
}

/// Utility type implementing [IntoAdvices] that emits a single code frame
/// advice with the provided path, span and source code
#[derive(Debug)]
pub struct FrameAdvice<Path, Span, SourceCode> {
    pub path: Path,
    pub span: Span,
    pub source_code: SourceCode,
}

impl<Path, Span, SourceCode> IntoAdvices for FrameAdvice<Path, Span, SourceCode>
where
    Path: AsPath,
    Span: AsSpan,
    SourceCode: AsSourceCode,
{
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        let location = Location::builder()
            .path(&self.path)
            .span(&self.span)
            .source_code(&self.source_code)
            .build();

        if let Some(location) = location {
            visitor.visit_frame(location)?;
        }

        Ok(())
    }
}

/// Utility type implementing [IntoAdvices] that emits a diff advice with the
/// provided prev and next text
#[derive(Debug)]
pub struct DiffAdvice<A, B> {
    pub prev: A,
    pub next: B,
}

impl<A, B> IntoAdvices for DiffAdvice<A, B>
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        visitor.visit_diff(self.prev.as_ref(), self.next.as_ref())
    }
}

/// Utility type implementing [IntoAdvices] that emits a command advice with
/// the provided text
#[derive(Debug)]
pub struct CommandAdvice<T> {
    pub command: T,
}

impl<T> IntoAdvices for CommandAdvice<T>
where
    T: AsRef<str>,
{
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        visitor.visit_command(self.command.as_ref())
    }
}
