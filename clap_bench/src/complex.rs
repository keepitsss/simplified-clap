use std::hint::black_box;

use clap::{arg, Command};

#[allow(unreachable_pub)]
pub fn create_app() -> Command {
    black_box(
        Command::new("claptests")
            .version("0.1")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .arg(arg!(-o --option <opt> ... "tests options"))
            .arg(arg!([positional] "tests positionals"))
            .arg(arg!(-f --flag ... "tests flags").global(true))
            .args([
                arg!(flag2: -F "tests flags with exclusions")
                    .conflicts_with("flag")
                    .requires("option2"),
                arg!(option2: --"long-option-2" <option2> "tests long options with exclusions")
                    .conflicts_with("option")
                    .requires("positional2"),
                arg!([positional2] "tests positionals with exclusions"),
                arg!(-O --Option <option3> "tests options with specific value sets")
                    .value_parser(["fast", "slow"]),
                arg!([positional3] ... "tests positionals with specific values")
                    .value_parser(["vi", "emacs"]),
                arg!(--multvals <s> "Tests multiple values not mult occs")
                    .value_names(["one", "two"]),
                arg!(
                    --multvalsmo <s> "Tests multiple values, not mult occs"
                )
                .required(false)
                .value_names(["one", "two"]),
                arg!(--minvals2 <minvals> ... "Tests 2 min vals").num_args(2..),
                arg!(--maxvals3 <maxvals> ... "Tests 3 max vals").num_args(1..=3),
            ])
            .subcommand(
                Command::new("subcmd")
                    .about("tests subcommands")
                    .version("0.1")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .arg(arg!(-o --option <scoption> ... "tests options"))
                    .arg(arg!([scpositional] "tests positionals")),
            ),
    )
}
#[allow(unreachable_pub)]
pub const ARGS: &[&[&str]] = black_box(&[
    &[""],
    &["myprog", "-f"],
    &["myprog", "-o", "option1"],
    &["myprog", "arg1"],
    &["myprog", "subcmd"],
    &["myprog", "subcmd", "-f"],
    &["myprog", "subcmd", "-o", "option1"],
    &["myprog", "subcmd", "arg1"],
    &["myprog", "subcmd", "-f", "-o", "option1", "arg1"],
    &[
        "myprog",
        "-ff",
        "-o",
        "option1",
        "arg1",
        "-O",
        "fast",
        "arg2",
        "--multvals",
        "one",
        "two",
        "emacs",
    ],
    &[
        "myprog",
        "arg1",
        "-f",
        "arg2",
        "--long-option-2",
        "some",
        "-O",
        "slow",
        "--multvalsmo",
        "one",
        "two",
        "--minvals2",
        "3",
        "2",
        "1",
    ],
]);
