use std::time::Duration;

use clap_bench::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn build(c: &mut Criterion) {
    c.bench_function("build args", |b| {
        b.iter(build_bench);
    });
}

fn startup(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(parse_bench);
    });
}

fn render_help(c: &mut Criterion) {
    c.bench_function("render help", |b| {
        b.iter(render_help_bench);
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = startup, render_help, build
);
criterion_main!(benches);
