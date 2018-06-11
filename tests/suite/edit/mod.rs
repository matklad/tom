use testutils::assert_eq_text;

use tom::{
    Factory, TomlFile,
    ast::AstNode,
};

mod cargo_toml;

use self::cargo_toml::CargoToml;


#[test]
fn adding_dependency_to_table() {
    check_cargo_toml_edit(
        |toml| {
            toml.add_dependency("pest", "1.0")
        },
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
    );
}

#[test]
fn adding_dependency_no_table() {
    check_cargo_toml_edit(
        |toml| toml.add_dependency("pest", "1.0"),
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
    );

    check_cargo_toml_edit(
        |toml| toml.add_dependency("pest", "1.0"),
        r#"
[package]
name = "tom"

[bin]
name = "baz"
"#,
        r#"
[package]
name = "tom"
[dependencies]
pest = "1.0"

[bin]
name = "baz"
"#,
    )
}

fn check_edit(before: &str, after: &str, edit: &Fn(&TomlFile) -> String) {
    let file = TomlFile::new(before.to_string());
    let actual = edit(&file);
    assert_eq_text(after, &actual);
}

fn check_cargo_toml_edit(
    f: impl FnOnce(&mut CargoToml),
    before: &str,
    after: &str,
) {
    let file = TomlFile::new(before.to_string());
    let factory = Factory::new();
    let mut cargo_toml = CargoToml::new(&file, &factory);
    f(&mut cargo_toml);
    let actual = cargo_toml.finish();
    assert_eq_text(after, &actual)
}
