use std::{
    fs,
    path::{Path, PathBuf},
};

use difference::Changeset;

const REWRITE: bool = false;

pub fn print_difference(expected: &str, actual: &str, path: &Path) {
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
    // panic!("Comparison failed")
}

// pub fn assert_eq_text(expected: &str, actual: &str) {
//     if expected == actual {
//         return;
//     }
//     if expected.trim() == actual.trim() {
//         eprintln!("whitespace difference!");
//         eprintln!("expected:\n{:?}\nactual:\n{:?}\n", expected, actual);
//         panic!("Comparison failed");
//     }
//     let changeset = Changeset::new(actual, expected, "\n");
//     if expected.lines().count() < 20 {
//         let line = "--------------------------";
//         eprintln!(
//             "
// Expected:
// {line}
// {expected}
// {line}
// Actual:
// {line}
// {actual}
// {line}
// Diff:
// {diff}
// ",
//             line = line,
//             expected = expected,
//             actual = actual,
//             diff = changeset
//         );
//         panic!("Comparison failed")
//     } else {
//         print!("{}", changeset);
//         panic!("Comparison failed")
//     }
// }

fn project_dir() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir)
}

pub fn test_data_dir() -> PathBuf {
    project_dir().join("tests/data")
}
