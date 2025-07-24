use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

// use std::hint::black_box;
// use clap::{arg, Command};

mod complex;
mod empty;
mod ripgrep;
mod rustup;
mod simple;

fn build(c: &mut Criterion) {
    c.bench_function("build args", |b| {
        b.iter(|| {
            complex::create_app();
            empty::create_app();
            ripgrep::app_short();
            ripgrep::app_long();
            simple::create_app();
            rustup::build_cli();
        });
    });
}

fn startup(c: &mut Criterion) {
    c.bench_function("startup", |b| {
        b.iter(|| {
            for args in complex::ARGS {
                complex::create_app().get_matches_from(black_box(*args));
            }
            for args in empty::ARGS {
                empty::create_app().get_matches_from(black_box(*args));
            }
            for args in ripgrep::ARGS {
                ripgrep::app_short().get_matches_from(black_box(*args));
            }
            for args in ripgrep::ARGS {
                ripgrep::app_long().get_matches_from(black_box(*args));
            }
            for args in simple::ARGS {
                simple::create_app().get_matches_from(black_box(*args));
            }
            for args in rustup::ARGS {
                rustup::build_cli().get_matches_from(black_box(*args));
            }
        });
    });
}

fn render_help(c: &mut Criterion) {
    c.bench_function("render help", |b| {
        b.iter(|| {
            complex::create_app().render_help().to_string();
            empty::create_app().render_help().to_string();
            ripgrep::app_long().render_help().to_string();
            ripgrep::app_short().render_help().to_string();
            simple::create_app().render_help().to_string();
            rustup::build_cli().render_help().to_string();
        });
    });
}

criterion_group!(benches, startup, render_help, build);
criterion_main!(benches);
