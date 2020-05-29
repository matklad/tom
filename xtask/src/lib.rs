//! Support library for `cargo xtask` command.
//!
//! See https://github.com/matklad/cargo-xtask/

pub mod codegen;
pub mod not_bash;
pub mod pre_commit;
use anyhow::{ensure, Result};

use std::{
    env,
    path::{Path, PathBuf},
    fmt,
};
use not_bash::run;

pub fn project_root_dir() -> PathBuf {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    Path::new(&manifest_dir).parent().unwrap().to_owned()
}

fn reformat(text: impl fmt::Display) -> Result<String> {
    let stdout = run!(
        "rustfmt --config-path {}", project_root_dir().join("rustfmt.toml").display();
        <text.to_string().as_bytes()
    )?;
    let preamble = "Generated file, do not edit by hand, see `xtask/src/codegen`";
    Ok(format!("//! {}\n\n{}\n", preamble, stdout))
}

pub fn lint() -> Result<()> {
    ensure!(
        run!("cargo clippy --version").is_ok(),
        "Failed run cargo clippy. \
        Please run `rustup component add clippy` to install it."
    );

    let allowed_lints = ["clippy::cognitive_complexity"];
    run!(
        "cargo clippy --all-features --all-targets -- -A {}",
        allowed_lints.join(" -A ")
    )?;
    Ok(())
}
