use std::hint::black_box;

use clap::{arg, Command};

#[allow(unreachable_pub)]
pub fn create_app() -> Command {
    black_box(
        Command::new("claptests")
            .version("0.1")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .arg(arg!(-f --flag         "tests flags"))
            .arg(arg!(-o --option <opt> "tests options"))
            .arg(arg!([positional]      "tests positional")),
    )
}

#[allow(unreachable_pub)]
pub const ARGS: &[&[&str]] = black_box(&[
    &["myprog", "-f"],
    &["myprog", "-o", "option1"],
    &["myprog", "arg1"],
]);
