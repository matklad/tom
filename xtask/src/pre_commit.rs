//! pre-commit hook for code formatting

use anyhow::{bail, Result};
use std::{collections::HashSet, env::consts, path::Path};

use crate::{
    not_bash::{fs2, run},
    project_root_dir,
};

pub fn install_hook(should_overwrite: bool) -> Result<()> {
    let hook_path = project_root_dir().join(format!(".git/hooks/pre-commit{}", consts::EXE_SUFFIX));

    if hook_path.exists() {
        if should_overwrite {
            eprintln!("Overwriting the existing git pre-commit hook (due to --force)...");
        } else {
            bail!("Git pre-commit hook is already created (use --force to overwrite)");
        }
    }

    let me = std::env::current_exe()?;
    fs2::copy(me, hook_path)?;

    Ok(())
}

pub fn run_hook() -> Result<()> {
    let diff = run!("git diff --diff-filter=MAR --name-only --cached")?;
    let staged_files = || diff.lines().map(Path::new);

    let packages = package_names_from_staged_files(&mut staged_files());

    if packages.is_empty() {
        return Ok(());
    }

    run!("cargo fmt --package {}", packages.join(" "))?;

    // Stage changes introduced by rustfmt:

    let root = project_root_dir();
    for changed_file_path in diff.lines() {
        run!(
            "git update-index --add {}",
            root.join(changed_file_path).display()
        )?;
    }
    Ok(())
}

fn package_names_from_staged_files(
    staged_files: &mut dyn Iterator<Item = &'_ Path>,
) -> Vec<String> {
    let package_dirs: HashSet<_> = staged_files
        .filter(|it| it.extension().map(|it| it == "rs").unwrap_or(false))
        .map(|it| {
            if it.starts_with("xtask") {
                project_root_dir().join("xtask")
            } else {
                assert!(it.starts_with("crates"));
                let crate_name = it.components().nth(1).unwrap();
                project_root_dir().join("crates").join(crate_name)
            }
        })
        .collect();

    package_dirs
        .into_iter()
        .map(|it| {
            let cargo_toml = it.join("Cargo.toml");
            let cargo_toml = fs2::read_to_string(&cargo_toml).unwrap();
            let name = cargo_toml.find("name = \"").unwrap() + "name = \"".len();
            let len = cargo_toml[name..].find('"').unwrap();
            cargo_toml[name..name + len].to_owned()
        })
        .collect()
}
