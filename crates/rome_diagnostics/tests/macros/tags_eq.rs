use rome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(tags = Identifier)]
struct TestDiagnostic {}

fn main() {}
