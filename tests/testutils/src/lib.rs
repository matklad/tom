extern crate difference;

use std::{
    path::{Path, PathBuf},
    fs::{self, read_dir},
};

use difference::Changeset;

/// Read file and normalize newlines.
///
/// `rustc` seems to always normalize `\r\n` newlines to `\n`:
///
/// ```
/// let s = "
/// ";
/// assert_eq!(s.as_bytes(), &[10]);
/// ```
///
/// so this should always be correct.
fn read_text(path: &Path) -> String {
    fs::read_to_string(path).unwrap().replace("\r\n", "\n")
}

pub fn dir_tests<F>(paths: &[&str], f: F)
    where
        F: Fn(&str) -> String,
{
    for path in collect_tests(paths) {
        let input_code = read_text(&path);
        let cst = f(&input_code);
        let path = path.with_extension("txt");
        if !path.exists() {
            println!("\nfile: {}", path.display());
            println!("No .txt file with expected result, creating...\n");
            println!("{}\n{}", input_code, cst);
            fs::write(&path, cst).unwrap();
            panic!("No expected result")
        }
        let expected = read_text(&path);
        let expected = expected.as_str();
        let cst = cst.as_str();
        assert_equal_text(expected, cst, &path);
    }
}

fn assert_equal_text(expected: &str, actual: &str, path: &Path) {
    if expected != actual {
        print_difference(expected, actual, path)
    }
}

fn collect_tests(paths: &[&str]) -> Vec<PathBuf> {
    paths
        .iter()
        .flat_map(|path| {
            let path = test_data_dir().join(path);
            test_from_dir(&path).into_iter()
        })
        .collect()
}

fn test_from_dir(dir: &Path) -> Vec<PathBuf> {
    let mut acc = Vec::new();
    for file in read_dir(&dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().unwrap_or_default() == "toml" {
            acc.push(path);
        }
    }
    acc.sort();
    acc
}

const REWRITE: bool = false;

fn print_difference(expected: &str, actual: &str, path: &Path) {
    let dir = project_dir();
    let path = path.strip_prefix(&dir).unwrap_or_else(|_| path);
    if expected.trim() == actual.trim() {
        println!("whitespace difference, rewriting");
        println!("file: {}\n", path.display());
        fs::write(path, actual).unwrap();
        return;
    }
    if REWRITE {
        println!("rewriting {}", path.display());
        fs::write(path, actual).unwrap();
        return;
    }
    let changeset = Changeset::new(actual, expected, "\n");
    print!("{}", changeset);
    println!("file: {}\n", path.display());
    panic!("Comparison failed")
}

pub fn assert_eq_text(expected: &str, actual: &str) {
    if expected == actual {
        return;
    }
    if expected.trim() == actual.trim() {
        eprintln!("whitespace difference!");
        eprintln!("expected:\n{:?}\nactual:\n{:?}\n", expected, actual);
        panic!("Comparison failed");
    }
    let changeset = Changeset::new(actual, expected, "\n");
    if expected.lines().count() < 20 {
        let line = "--------------------------";
        eprintln!("
Expected:
{line}
{expected}
{line}
Actual:
{line}
{actual}
{line}
Diff:
{diff}
",
                  line = line,
                  expected = expected,
                  actual = actual,
                  diff = changeset);
        panic!("Comparison failed")
    } else {
        print!("{}", changeset);
        panic!("Comparison failed")
    }
}

fn project_dir() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

fn test_data_dir() -> PathBuf {
    project_dir().join("tests/data")
}
