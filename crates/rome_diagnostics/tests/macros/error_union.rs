use rome_diagnostics::v2::Diagnostic;

#[derive(Diagnostic)]
union ErrorUnion {
    int: u32,
    float: f32,
}

fn main() {}
