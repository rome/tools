use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(description = "text {unclosed"))]
struct TestDiagnostic {}

fn main() {}
