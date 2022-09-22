use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(unknown))]
struct TestDiagnostic {}

fn main() {}
