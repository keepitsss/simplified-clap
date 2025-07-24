use clap_bench::*;

fn main() {
    for _ in 0..2000 {
        build_bench();
        render_help_bench();
        parse_bench();
    }
}
