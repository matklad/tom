extern crate tom;
extern crate testutils;

use tom::{TomlFile, Factory};
use tom::ast::AstNode;
use testutils::assert_eq_text;


fn add_dependency(file: &TomlFile, name: &str, version: &str) -> String {
    let f = Factory::new();
    let mut edit = file.edit();
    let deps = file.ast().find_table(&["dependencies"])
        .unwrap_or_else(|| {
            let table = f.table(&mut ["dependencies"].iter().cloned(), &mut [].iter().cloned());
            edit.append_child(file.ast().node(), table.node());
            table
        });
    let version = f.val_string(version);
    let dep = f.key_val(name, version);
    edit.append_child(deps.node(), dep.node());
    edit.finish()
}

#[test]
fn adding_dependency_to_table() {
    check_edit(
        r#"
[package]
name = "tom"

[dependencies]
lalrpop-util = "0.15"
regex = "0.2"
"#,
        r#"
[package]
name = "tom"

[dependencies]
lalrpop-util = "0.15"
regex = "0.2"
pest = "1.0"
"#,
        &|file| add_dependency(file, "pest", "1.0"),
    );
}

#[test]
fn adding_dependency_no_table() {
    check_edit(
        r#"
[package]
name = "tom"
"#,
        r#"
[package]
name = "tom"
[dependencies]
pest = "1.0"
"#,
        &|file| add_dependency(file, "pest", "1.0"),
    );
}



fn check_edit(
    before: &str,
    after: &str,
    edit: &Fn(&TomlFile) -> String,
) {
    let file = TomlFile::new(before.to_string());
    let actual = edit(&file);
    assert_eq_text(after, &actual);
}
