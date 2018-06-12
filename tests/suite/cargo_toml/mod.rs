mod manipulator;

use self::manipulator::CargoTomlManipulator;
use check_edit;

#[test]
fn adding_dependency_to_table() {
    check_cargo_toml_edit(
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
        |toml| toml.add_dependency("pest", "1.0"),
    );
}

#[test]
fn adding_dependency_no_table() {
    check_cargo_toml_edit(
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
        |toml| toml.add_dependency("pest", "1.0"),
    );
}

#[test]
fn adding_dependency_no_table_bin_section() {
    check_cargo_toml_edit(
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
        |toml| toml.add_dependency("pest", "1.0"),
    )
}

#[test]
fn adding_two_dependencies() {
    check_cargo_toml_edit(
        r#"
[package]
name = "tom"
[dependencies]
"#,
        r#"
[package]
name = "tom"
[dependencies]
regex = "1.0"
pest = "1.0"
"#,
        |toml| {
            toml.add_dependency("regex", "1.0");
            toml.add_dependency("pest", "1.0");
        },
    );
}


fn check_cargo_toml_edit(
    before: &str,
    after: &str,
    edit: impl FnOnce(&mut CargoTomlManipulator),
) {
    check_edit(before, after, |file| {
        let factory = Factory::new();
        let mut cargo_toml = CargoTomlManipulator::new(file, &factory);
        edit(&mut cargo_toml);
        cargo_toml.finish()
    })
}

