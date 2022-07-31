mod contains;

use contains::*;
use criterion::{criterion_group, criterion_main, Criterion};
use fastbloom_rs::Membership;

fn criterion_benchmark(c: &mut Criterion) {
    let set = contains_hashset_setup();
    c.bench_function("contains_hashset", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in search_for() {
                count += if set.contains(*k) { 1 } else { 0 };
            }
            count
        })
    });

    let set = contains_btreeset_setup();
    c.bench_function("contains_btreeset", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in search_for() {
                count = if set.contains(*k) { 1 } else { 0 };
            }
            count
        })
    });

    let set = contains_bloom_setup();
    c.bench_function("contains_bloom", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in search_for() {
                count += if set.contains(k.as_bytes()) { 1 } else { 0 };
            }
            count
        })
    });

    let set = contains_trie_setup();
    c.bench_function("contains_trie", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in search_for() {
                count += if set.contains_key(k.as_bytes()) { 1 } else { 0 };
            }
            count
        })
    });

    let set = contains_slice_setup();
    c.bench_function("contains_slice", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in search_for() {
                count += set.iter().position(|x| x == k).unwrap_or(0);
            }
            count
        })
    });

    let set = contains_fst_setup();
    c.bench_function("contains_fst", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in search_for() {
                count += if set.contains(k) { 1 } else { 0 };
            }
            count
        })
    });

    let mut set = keywords();
    set.sort();
    c.bench_function("contains_binary_search", |b| {
        b.iter(|| {
            let mut count = 0;
            for k in search_for() {
                count += set.binary_search_by(|v| (*k).cmp(v.as_str())).unwrap_or(1);
            }
            count
        })
    });
}

criterion_group!(contains, criterion_benchmark);
criterion_main!(contains);
