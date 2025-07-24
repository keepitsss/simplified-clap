use std::hint::black_box;

use clap::Command;

#[allow(unreachable_pub)]
pub fn create_app() -> Command {
    black_box(Command::new("claptests"))
}
#[allow(unreachable_pub)]
pub const ARGS: &[&[&str]] = &[&[""]];
