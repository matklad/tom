//! A bad shell -- small cross platform module for writing glue code

use anyhow::{Context, Result, bail};

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

    pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64> {
        let from = from.as_ref();
        let to = to.as_ref();
        fs::copy(from, to)
            .with_context(|| format!("Failed to copy {} to {}", from.display(), to.display()))
    }
}

macro_rules! _run {
    ($($expr:expr),*) => {
        run!($($expr),*; echo = true)
    };
    ($($expr:expr),* ; echo = $echo:expr) => {
        $crate::not_bash::run_process(format!($($expr),*), $echo, None)
    };
    ($($expr:expr),* ; <$stdin:expr) => {
        $crate::not_bash::run_process(format!($($expr),*), false, Some($stdin))
    };
}
pub(crate) use _run as run;
use std::{
    io::Write,
    process::{Stdio, Command},
};

#[doc(hidden)]
pub fn run_process(cmd: String, echo: bool, stdin: Option<&[u8]>) -> Result<String> {
    run_process_inner(&cmd, echo, stdin).with_context(|| format!("process `{}` failed", cmd))
}

fn run_process_inner(cmd: &str, echo: bool, stdin: Option<&[u8]>) -> Result<String> {
    let mut args = shelx(cmd);
    let binary = args.remove(0);

    if echo {
        println!("> {}", cmd)
    }

    let mut command = Command::new(binary);
    command.args(args).stderr(Stdio::inherit());
    let output = match stdin {
        None => command.stdin(Stdio::null()).output(),
        Some(stdin) => {
            command.stdin(Stdio::piped()).stdout(Stdio::piped());
            let mut process = command.spawn()?;
            process.stdin.take().unwrap().write_all(stdin)?;
            process.wait_with_output()
        }
    }?;
    let stdout = String::from_utf8(output.stdout)?;

    if echo {
        print!("{}", stdout)
    }

    if !output.status.success() {
        bail!("{}", output.status)
    }

    Ok(stdout.trim().to_string())
}

// Some fake shell lexing
fn shelx(cmd: &str) -> Vec<String> {
    cmd.split_whitespace().map(|it| it.to_string()).collect()
}
