use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use jfrview::interpret_jfr;

const BYTES: &[u8] = include_bytes!("../test-data/heavy.jfr");

pub fn heavy_parse(c: &mut Criterion) {
    c.bench_with_input(BenchmarkId::new("Heavy.jfr", false), &false, |b, _i| {
        b.iter(|| interpret_jfr(BYTES.into()));
    });
    c.bench_with_input(BenchmarkId::new("Heavy.jfr", true), &true, |b, _i| {
        b.iter(|| interpret_jfr(BYTES.into()));
    });
}

criterion_group!(benches, heavy_parse);
criterion_main!(benches);
