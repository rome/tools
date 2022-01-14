use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rslint_regex::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse regex", |b| {
        b.iter(|| {
            let parser = Parser::new_from_pattern_and_flags(
                r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$",
                0,
                0,
                EcmaVersion::ES2021,
                false,
                Flags::empty(),
            );
            black_box(parser.parse().unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
