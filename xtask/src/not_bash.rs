//! A bad shell -- small cross platform module for writing glue code

use anyhow::{Context, Result};

pub mod fs2 {
    use super::*;
    use std::{fs, path::Path};

    // Enhance standard lib with better error messages:

    pub fn read_dir(path: impl AsRef<Path>) -> Result<fs::ReadDir> {
        let path = path.as_ref();
        fs::read_dir(path).with_context(|| format!("Failed to read dir {}", path.display()))
    }

    pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
        let path = path.as_ref();
        fs::read_to_string(path).with_context(|| format!("Failed to read file {}", path.display()))
    }

    pub fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
        let path = path.as_ref();
        fs::write(path, contents)
            .with_context(|| format!("Failed to write file {}", path.display()))
    }
}

// macro_rules! _run {
//     ($($expr:expr),*) => {
//         run!($($expr),*; echo = true)
//     };
//     ($($expr:expr),* ; echo = $echo:expr) => {
//         $crate::not_bash::run_process(format!($($expr),*), $echo)
//     };
// }
// pub(crate) use _run as run;
// use std::process::{Stdio, Command};

// #[doc(hidden)]
// pub(crate) fn run_process(cmd: String, echo: bool) -> Result<String> {
//     run_process_inner(&cmd, echo).with_context(|| format!("process `{}` failed", cmd))
// }

// fn run_process_inner(cmd: &str, echo: bool) -> Result<String> {
//     let mut args = shelx(cmd);
//     let binary = args.remove(0);

//     if echo {
//         println!("> {}", cmd)
//     }

//     let output = Command::new(binary)
//         .args(args)
//         .stdin(Stdio::null())
//         .stderr(Stdio::inherit())
//         .output()?;
//     let stdout = String::from_utf8(output.stdout)?;

//     if echo {
//         print!("{}", stdout)
//     }

//     ensure!(output.status.success(), "{}", output.status);

//     Ok(stdout.trim().to_string())
// }

// // Some fake shell lexing
// fn shelx(cmd: &str) -> Vec<String> {
//     cmd.split_whitespace().map(|it| it.to_string()).collect()
// }