//! See https://github.com/matklad/cargo-xtask/.
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`. Notably, it provides `cargo xtask codegen`
//! for code generation.
//!
//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

use anyhow::Result;
use xtask::{pre_commit, codegen, lint};
use std::env;

const USAGE: &str = "\
xtask

USAGE:
    xtask <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    codegen                       Run code generation
    lint                          Run the linter against the project, it enforces best practices
    install-pre-commit-hook       Install git hook to automatically format code before each commit";

fn main() -> Result<()> {
    if let Some(true) = env::args().next().map(|it| it.contains("pre-commit")) {
        return pre_commit::run_hook();
    }

    let mut args = pico_args::Arguments::from_env();

    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "codegen" => {
            codegen::gen_ast(codegen::Mode::Overwrite)?;
            codegen::gen_symbols(codegen::Mode::Overwrite)?;
            codegen::gen_parser_tests(codegen::Mode::Overwrite)?;
        }
        "lint" => {
            lint()?;
        }
        "install-pre-commit-hook" => {
            let force = args.contains("--force");
            args.finish()?;
            pre_commit::install_hook(force)?;
            eprintln!("Git pre-commit hook is installed. Have a nice day :D")
        }
        _ => eprintln!("{}", USAGE),
    }
    Ok(())
}
