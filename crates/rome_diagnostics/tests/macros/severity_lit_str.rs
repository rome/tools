use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(severity = "Error")]
struct TestDiagnostic {}

fn main() {}
