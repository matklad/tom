extern crate tom;
extern crate testutils;

use tom::{TomlFile, Factory};
use tom::ast::AstNode;
use testutils::assert_eq_text;


fn add_dependency(file: &TomlFile, name: &str, version: &str) -> String {
    let deps = file.ast().find_table(&["dependencies"]).unwrap();
    let mut edit = file.edit();
    let f = Factory::new();
    let version = f.val_string(version);
    let dep = f.key_val(name, version);
    edit.append_child(deps.node(), dep.node());
    edit.finish()
}

#[test]
fn adding_dependency() {
    let before = r#"[package]
name = "tom"
version = "0.1.0"
authors = ["Aleksey Kladov <aleksey.kladov@gmail.com>"]

[dependencies]
lalrpop-util = "0.15"
regex = "0.2"

[build-dependencies]
file = "1.0"
heck = "0.1.0"
lalrpop = "0.15"

[dev-dependencies.testutils]
path = "./tests/testutils"
"#;
    let file = TomlFile::new(before.to_string());
    let actual = add_dependency(&file, "pest", "1.0");

    let after = r#"[package]
name = "tom"
version = "0.1.0"
authors = ["Aleksey Kladov <aleksey.kladov@gmail.com>"]

[dependencies]
lalrpop-util = "0.15"
regex = "0.2"
pest = "1.0"

[build-dependencies]
file = "1.0"
heck = "0.1.0"
lalrpop = "0.15"

[dev-dependencies.testutils]
path = "./tests/testutils"
"#;
    assert_eq_text(after, &actual);
}
