use {CargoToml, Dependency, DependencySource};

#[test]
fn updating_dependency_no_table() {
    check_cargo_toml_edit(
        r#"
[package]
name = "tom"
"#,
        r#"
[package]
name = "tom"

[dependencies]
regex = "1.0"
"#,
        |toml| {
            toml.update_dependency(&Dependency {
                name: "regex".to_string(),
                source: DependencySource::Version("1.0".to_string()),
                optional: false,
            }).unwrap();
        },
    );

    check_cargo_toml_edit(
        r#"
[package]
name = "tom"

[bin]
name = "bar"
"#,
        r#"
[package]
name = "tom"

[dependencies]
regex = "1.0"

[bin]
name = "bar"
"#,
        |toml| {
            toml.update_dependency(&Dependency {
                name: "regex".to_string(),
                source: DependencySource::Version("1.0".to_string()),
                optional: false,
            }).unwrap();
        },
    );
}

#[test]
fn updating_dependency() {
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
"#,
        |toml| {
            toml.update_dependency(&Dependency {
                name: "regex".to_string(),
                source: DependencySource::Version("1.0".to_string()),
                optional: false,
            }).unwrap();
        },
    );

    check_cargo_toml_edit(
        r#"
[package]
name = "tom"

[dependencies]
regex = "1.0"
"#,
        r#"
[package]
name = "tom"

[dependencies]
regex = { git = "http://example.com" }
"#,
        |toml| {
            toml.update_dependency(&Dependency {
                name: "regex".to_string(),
                source: DependencySource::Git {
                    url: "http://example.com".to_string(),
                    version: None,
                    branch: None,
                },
                optional: false,
            }).unwrap();
        },
    );

    check_cargo_toml_edit(
        r#"
[package]
name = "tom"

[dependencies.regex]
version = "1.0"
"#,
        r#"
[package]
name = "tom"

[dependencies.regex]
version = "1.0"
git = "http://example.com"
"#,
        |toml| {
            toml.update_dependency(&Dependency {
                name: "regex".to_string(),
                source: DependencySource::Git {
                    url: "http://example.com".to_string(),
                    version: None,
                    branch: None,
                },
                optional: false,
            }).unwrap();
        },
    );

    check_cargo_toml_edit(
        r#"
[package]
name = "tom"

[dependencies]
regex = { git = "http://example.com" }
"#,
        r#"
[package]
name = "tom"

[dependencies]
regex = { git = "http://example.com", branch = "dev" }
"#,
        |toml| {
            toml.update_dependency(&Dependency {
                name: "regex".to_string(),
                source: DependencySource::Git {
                    url: "http://example.com".to_string(),
                    version: None,
                    branch: Some("dev".to_string()),
                },
                optional: false,
            }).unwrap();
        },
    );
}

fn check_cargo_toml_edit(before: &str, after: &str, edit: impl FnOnce(&mut CargoToml)) {
    let mut cargo_toml = CargoToml::new(before).unwrap();
    edit(&mut cargo_toml);
    let actual = cargo_toml.text();
    assert_eq!(after, &actual);
}
