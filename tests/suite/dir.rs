// use serde_json;
use std::{
    fs::{self, read_dir},
    path::{Path, PathBuf},
};
use tom::{TomlDoc};
use util::{print_difference, test_data_dir};

enum ExpectErrors {
    Yes,
    No,
    Maybe,
}

fn dir_tests(paths: &[&str], expect_errors: ExpectErrors) {
    for path in collect_tests(paths) {
        // Load TOML document.
        let input_code = read_text(&path);
        let doc = TomlDoc::new(&input_code);
        match expect_errors {
            ExpectErrors::Yes => {
                if doc.errors().is_empty() {
                    panic!(
                        "Expected errors, but none found in `{}`:\n{}",
                        path.display(),
                        input_code
                    );
                }
            }
            ExpectErrors::No => {
                if !doc.errors().is_empty() {
                    panic!(
                        "No errors expected in `{}`, but found:\n{:?}",
                        path.display(),
                        doc.errors()
                    );
                }
            }
            ExpectErrors::Maybe => {}
        }

        // Check CST against .txt file.
        let cst = doc.debug();
        let cst_path = path.with_extension("txt");
        if !cst_path.exists() {
            println!("\nfile: {}", cst_path.display());
            println!("No .txt file with expected result, creating...\n");
            println!("{}\n{}", input_code, cst);
            fs::write(&cst_path, cst).unwrap();
            panic!("No expected result");
        }
        let expected = read_text(&cst_path);
        if expected != cst {
            print_difference(&expected, &cst, &cst_path)
        }

        // // Check model against .json file.
        // // TODO: Remove this once the API is stable enough.
        // let actual_json = match ::std::panic::catch_unwind(|| to_json(Item::Map(doc.model()))) {
        //     Ok(json) => json,
        //     Err(e) => {
        //         let msg = match e.downcast_ref::<&'static str>() {
        //             Some(s) => *s,
        //             None => match e.downcast_ref::<String>() {
        //                 Some(s) => &s[..],
        //                 None => "Box<Any>",
        //             },
        //         };
        //         println!(
        //             "Failed to convert to model to json for `{}`: {}",
        //             path.display(),
        //             msg
        //         );
        //         continue;
        //     }
        // };
        // // let actual_json = to_json(Item::Map(doc.model()));
        // let actual_pretty = serde_json::to_string_pretty(&actual_json).unwrap();
        // let json_path = path.with_extension("json");
        // if !json_path.exists() {
        //     // TODO: Remove this once model is more reliable.
        //     println!("\nJSON not yet ready: {}", json_path.display());
        //     continue;
        //     // println!("\nfile: {}", json_path.display());
        //     // println!("No .json file with expected result, creating...\n");
        //     // let pretty_json = serde_json::to_string_pretty(&actual_json).unwrap();
        //     // println!("{}\n{}", input_code, pretty_json);
        //     // fs::write(&json_path, pretty_json).unwrap();
        //     // panic!("No expected result");
        // }
        // let expected = read_text(&json_path);
        // let json: serde_json::Value = serde_json::from_str(&expected).expect("valid json");
        // let expected_pretty = serde_json::to_string_pretty(&json).unwrap();
        // if actual_json != json {
        //     print_difference(&expected_pretty, &actual_pretty, &json_path);
        // }
        // TODO: Uncomment this if you want to test the `Display` impl
        // produces valid JSON.
        // if actual_pretty != expected_pretty {
        //     print_difference(&expected_pretty, &actual_pretty, &json_path);
        // }
    }
}

// fn to_json(model: Item) -> serde_json::Value {
//     fn entry(ty: &str, value: serde_json::Value) -> serde_json::Value {
//         let mut map = serde_json::Map::new();
//         map.insert(
//             "type".to_string(),
//             serde_json::Value::String(ty.to_string()),
//         );
//         map.insert("value".to_string(), value);
//         serde_json::Value::Object(map)
//     }

//     match model {
//         Item::Map(map) => {
//             let mut json_map = serde_json::Map::new();
//             // TODO: add into_iter
//             for (k, v) in map.into_iter() {
//                 json_map.insert(k.to_string(), to_json(v));
//             }
//             serde_json::Value::Object(json_map)
//         }
//         Item::Array(items) => {
//             let json_items = items.into_iter().map(to_json).collect();
//             entry("array", json_items)
//         }
//         Item::Integer(i) => entry("integer", serde_json::Value::String(i.to_string())),
//         Item::Float(f) => {
//             let s = format!("{:.15}", f);
//             let s = format!("{}", s.trim_right_matches('0'));
//             let f = if s.ends_with('.') {
//                 format!("{}0", s)
//             } else {
//                 s
//             };
//             entry("float", serde_json::Value::String(f))
//         }
//         Item::Bool(b) => entry("bool", serde_json::Value::String(format!("{}", b))),
//         Item::DateTime => {
//             unimplemented!();
//         }
//         Item::String(s) => entry("string", serde_json::Value::String(s)),
//     }
// }

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
        } else if path.is_file() && !path.with_extension("toml").exists() {
            panic!("{} is missing a .toml file", path.display());
        }
    }
    acc.sort();
    acc
}

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

#[test]
fn test_parser_inline() {
    dir_tests(&["inline"], ExpectErrors::Maybe)
}

#[test]
fn test_parser_ok() {
    dir_tests(&["ok"], ExpectErrors::No);
}

#[test]
fn test_parser_err() {
    dir_tests(&["err"], ExpectErrors::Yes);
}

#[test]
fn test_parser_validation() {
    dir_tests(&["validation"], ExpectErrors::Yes)
}
