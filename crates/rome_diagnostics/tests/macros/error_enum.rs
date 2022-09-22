use rome_diagnostics::v2::Diagnostic;

#[derive(Debug, Diagnostic)]
enum ErrorEnum {
    Int(u32),
    Float(f32),
}

fn main() {}
