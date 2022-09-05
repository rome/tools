use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(unknown_attr)]
struct TestDiagnostic {}

fn main() {}
