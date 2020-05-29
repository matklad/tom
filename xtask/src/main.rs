//! See https://github.com/matklad/cargo-xtask/.
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`. Notably, it provides `cargo xtask codegen`
//! for code generation.
//!
//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

use anyhow::Result;
use xtask::codegen;


const USAGE: &str = "\
xtask

USAGE:
    xtask <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    codegen
    lint";

fn main() -> Result<()> {
    let mut args = pico_args::Arguments::from_env();

    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "codegen" => {
            codegen::gen_ast(codegen::Mode::Overwrite)?;
            codegen::gen_symbols(codegen::Mode::Overwrite)?;
            codegen::gen_parser_tests(codegen::Mode::Overwrite)?;
        },
        "lint" => {
            xtask::lint()?;
        }
        _ => eprintln!("{}", USAGE)
    }
    Ok(())
}
