use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(description("description")))]
struct TestDiagnostic {}

fn main() {}
