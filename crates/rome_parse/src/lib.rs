use rome_rowan::TextSize;

pub type ParseDiagnostic = rome_diagnostics::Diagnostic;

/// An abstraction for syntax tree implementations
pub trait TreeSink {
    type Kind;
    /// Adds new token to the current branch.
    fn token(&mut self, kind: Self::Kind, end: TextSize);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: Self::Kind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    /// Emit errors
    fn errors(&mut self, errors: Vec<ParseDiagnostic>);
}
