use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(description = Ident))]
struct TestDiagnostic {}

fn main() {}
