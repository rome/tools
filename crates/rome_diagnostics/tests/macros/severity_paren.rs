use rome_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(severity(Error))]
struct TestDiagnostic {}

fn main() {}
