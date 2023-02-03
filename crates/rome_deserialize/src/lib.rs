mod diagnostics;
mod visitor;

pub mod json;
pub use diagnostics::{DeserializationAdvice, DeserializationDiagnostic};
use rome_diagnostics::{DiagnosticExt, Error};
pub use visitor::VisitNode;

/// A small type to interrogate the result of a JSON deserialization
#[derive(Default)]
pub struct Deserialized<P> {
    diagnostics: Vec<Error>,
    deserialized: P,
}

impl<P> Deserialized<P> {
    /// [DeserializationDiagnostic] are converted into [Error]
    pub fn new(deserialized: P, diagnostics: Vec<DeserializationDiagnostic>) -> Self {
        Self {
            deserialized,
            diagnostics: diagnostics.into_iter().map(Error::from).collect(),
        }
    }

    /// Consumes self to return the diagnostics
    pub fn into_diagnostics(self) -> Vec<Error> {
        self.diagnostics
    }

    pub fn diagnostics(&self) -> &[Error] {
        self.diagnostics.as_slice()
    }

    /// Consumes self and returns the deserialized result
    pub fn into_deserialized(self) -> P {
        self.deserialized
    }

    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Consume itself to return the parsed result and its diagnostics
    pub fn consume(self) -> (P, Vec<Error>) {
        (self.deserialized, self.diagnostics)
    }

    /// It inject the file path to the current diagnostics
    pub fn with_file_path(self, path: &str) -> Self {
        let (deserialized, diagnostics) = self.consume();

        Deserialized {
            deserialized,
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.with_file_path(path))
                .collect(),
        }
    }
}
