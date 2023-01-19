mod diagnostics;
mod visitor;

pub mod json;
pub use diagnostics::{DeserializationAdvice, DeserializationDiagnostic};
use rome_diagnostics::Error;
pub use visitor::VisitConfigurationNode;

/// A small type to interrogate the result of a JSON deserialization
#[derive(Default)]
pub struct Deserialized<P> {
    diagnostics: Vec<Error>,
    parsed: P,
}

impl<P> Deserialized<P> {
    /// [DeserializationDiagnostic] are converted into [Error]
    pub fn new(parsed: P, diagnostics: Vec<DeserializationDiagnostic>) -> Self {
        Self {
            parsed,
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

    pub fn into_deserialized(self) -> P {
        self.parsed
    }

    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Consume itself to return the parsed result and its diagnostics
    pub fn consume(self) -> (P, Vec<Error>) {
        (self.parsed, self.diagnostics)
    }
}
