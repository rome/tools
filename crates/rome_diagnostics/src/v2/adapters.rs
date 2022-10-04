//! This modules exposes a number of "adapter diagnostics" that wrap error types
//! such as [std::error::Error] or [std::io::Error] in newtypes implementing the
//! [Diagnostic] trait

use std::io;

use rome_console::{fmt, markup};

use super::{category, Category, Diagnostic};

/// Implements [Diagnostic] over types implementing [std::error::Error].
#[derive(Debug)]
pub struct StdError {
    error: Box<dyn std::error::Error + Send + Sync>,
}

impl<E: std::error::Error + Send + Sync + 'static> From<E> for StdError {
    fn from(error: E) -> Self {
        Self {
            error: Box::new(error),
        }
    }
}

impl Diagnostic for StdError {
    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }
}

struct AsConsoleDisplay<'a, T>(&'a T);

impl<T: std::fmt::Display> fmt::Display for AsConsoleDisplay<'_, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

/// Implements [Diagnostic] over for [io::Error].
#[derive(Debug)]
pub struct IoError {
    error: io::Error,
}

impl From<io::Error> for IoError {
    fn from(error: io::Error) -> Self {
        Self { error }
    }
}

impl Diagnostic for IoError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let error = self.error.to_string();
        fmt.write_str(&error)
    }
}
