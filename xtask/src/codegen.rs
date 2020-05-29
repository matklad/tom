//! We use code generation heavily in tom.
//!
//! Rather then doing it via proc-macros, we use old-school way of just dumping
//! the source code.
//!
//! This module's submodules define specific bits that we generate.

mod gen_ast;
mod gen_parser_tests;
mod gen_symbols;

pub use gen_ast::gen_ast;
pub use gen_parser_tests::gen_parser_tests;
pub use gen_symbols::gen_symbols;
use std::path::Path;
use anyhow::{Result, ensure};
use crate::not_bash::fs2;

#[derive(Copy, Clone)]
pub enum Mode {
    Overwrite,
    Verify,
}

fn verify_or_overwrite(mode: Mode, file_path: &Path, expected: &str) -> Result<()> {
    match mode {
        Mode::Verify => {
            let actual = fs2::read_to_string(file_path)?;
            ensure!(
                normalize(&actual) == normalize(expected),
                "{}: generated code is stale, please run `cargo xtask codegen` \
                    or `cargo xtask install-pre-commit-hook [--force]` handle \
                    codegen updates automatically",
                file_path.display(),
            );
        }
        Mode::Overwrite => fs2::write(file_path, expected)?,
    }
    return Ok(());

    fn normalize(s: &str) -> String {
        s.replace("\r\n", "\n")
    }
}

const AST_NODES_OUT_FILE_PATH: &str = "crates/tom_syntax/src/ast/generated.rs";
const SYMBOLS_OUT_FILE_PATH: &str = "crates/tom_syntax/src/symbol/generated.rs";

const GRAMMAR_DIR_PATH: &str = "crates/tom_syntax/src/parser/grammar.rs";
const INLINE_TESTS_OUT_DIR_PATH: &str = "crates/tom_syntax/tests/data/inline";
