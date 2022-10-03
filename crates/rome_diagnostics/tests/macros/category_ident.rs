use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(category = Identifier)]
struct TestDiagnostic {}

fn main() {}
