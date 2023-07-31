use rome_diagnostics::Diagnostic;

#[derive(Diagnostic)]
union ErrorUnion {
    int: u32,
    float: f32,
}

fn main() {}
