use std::hint::black_box;

pub mod complex;
pub mod empty;
pub mod ripgrep;
pub mod rustup;
pub mod simple;

pub fn build_bench() {
    complex::create_app();
    empty::create_app();
    ripgrep::app_short();
    ripgrep::app_long();
    simple::create_app();
    rustup::build_cli();
}
pub fn parse_bench() {
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
}
pub fn render_help_bench() {
    complex::create_app().render_help().to_string();
    empty::create_app().render_help().to_string();
    ripgrep::app_long().render_help().to_string();
    ripgrep::app_short().render_help().to_string();
    simple::create_app().render_help().to_string();
    rustup::build_cli().render_help().to_string();
}
