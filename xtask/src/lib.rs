//! Support library for `cargo xtask` command.
//!
//! See https://github.com/matklad/cargo-xtask/

pub mod codegen;
pub mod not_bash;
use anyhow::{ensure, Result};

use std::{
    env,
    path::{Path, PathBuf},
};
use not_bash::run;

pub fn project_root_dir() -> PathBuf {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    Path::new(&manifest_dir).parent().unwrap().to_owned()
}

pub fn lint() -> Result<()> {
    ensure!(
        run!("cargo clippy --version").is_ok(),
        "Failed run cargo clippy. \
        Please run `rustup component add clippy` to install it."
    );

    let allowed_lints = [
        "clippy::cognitive_complexity"
    ];
    run!("cargo clippy --all-features --all-targets -- -A {}", allowed_lints.join(" -A "))?;
    Ok(())
}
