//! Support library for `cargo xtask` command.
//!
//! See https://github.com/matklad/cargo-xtask/

pub mod codegen;
pub mod not_bash;

use std::{
    env,
    path::{Path, PathBuf},
};
pub fn project_root_dir() -> PathBuf {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    Path::new(&manifest_dir).parent().unwrap().to_owned()
}
