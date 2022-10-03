use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
struct TestDiagnostic {
    #[unknown_attr]
    field: bool,
}

fn main() {}
