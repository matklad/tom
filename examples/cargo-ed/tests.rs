use {CargoToml, Dependency, DependencySource};

#[test]
fn test_get_dependencies() {
    fn do_check(toml: &str, expected: Vec<Dependency>) {
        let cargo_toml = CargoToml::new(toml).unwrap();
        assert_eq!(cargo_toml.dependencies(), expected);
    }

    do_check(r#"
[dependencies]
foo = "1.0"
bar = { git = "https://example.com" }

[dependencies.baz]
version = "2.0"
git = "https://example.com"
branch = "dev"
"#, vec![
        Dependency {
            name: "bar".to_string(),
            optional: false,
            source: DependencySource::Git {
                url: "https://example.com".to_string(),
                version: None,
                branch: None,
            },
        },
        Dependency {
            name: "baz".to_string(),
            optional: false,
            source: DependencySource::Git {
                url: "https://example.com".to_string(),
                version: Some("2.0".to_string()),
                branch: Some("dev".to_string()),
            },
        },
        Dependency {
            name: "foo".to_string(),
            optional: false,
            source: DependencySource::Version("1.0".to_string()),
        },
    ]);

    do_check(r#"
dependencies = { foo = "1.0", bar = { git = "https://example.com" } }
"#, vec![
        Dependency {
            name: "bar".to_string(),
            optional: false,
            source: DependencySource::Git {
                url: "https://example.com".to_string(),
                version: None,
                branch: None,
            },
        },
        Dependency {
            name: "foo".to_string(),
            optional: false,
            source: DependencySource::Version("1.0".to_string()),
        },
    ]);

    do_check(r#"
dependencies.foo = "1.0"
dependencies.bar.git = "https://example.com"
dependencies.bar.version = "2.0"
dependencies.baz = { version = "9.2" }
"#, vec![
        Dependency {
            name: "bar".to_string(),
            optional: false,
            source: DependencySource::Git {
                url: "https://example.com".to_string(),
                version: Some("2.0".to_string()),
                branch: None,
            },
        },
        Dependency {
            name: "baz".to_string(),
            optional: false,
            source: DependencySource::Version("9.2".to_string()),
        },
        Dependency {
            name: "foo".to_string(),
            optional: false,
            source: DependencySource::Version("1.0".to_string()),
        },
    ]);

    do_check(r#"
[dependencies]
bar.git = "https://example.com"
bar.version = "2.0"
"#, vec![Dependency {
        name: "bar".to_string(),
        optional: false,
        source: DependencySource::Git {
            url: "https://example.com".to_string(),
            version: Some("2.0".to_string()),
            branch: None,
        },
    }]);
}

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

//#[test]
//fn updating_dependency() {
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//"#,
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//regex = "1.0"
//"#,
//        |toml| {
//            toml.update_dependency(&Dependency {
//                name: "regex".to_string(),
//                source: DependencySource::Version("1.0".to_string()),
//                optional: false,
//            }).unwrap();
//        },
//    );
//
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//regex = "1.0"
//"#,
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//regex = { git = "http://example.com" }
//"#,
//        |toml| {
//            toml.update_dependency(&Dependency {
//                name: "regex".to_string(),
//                source: DependencySource::Git {
//                    url: "http://example.com".to_string(),
//                    version: None,
//                    branch: None,
//                },
//                optional: false,
//            }).unwrap();
//        },
//    );
//
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//
//[dependencies.regex]
//version = "1.0"
//"#,
//        r#"
//[package]
//name = "tom"
//
//[dependencies.regex]
//version = "1.0"
//git = "http://example.com"
//"#,
//        |toml| {
//            toml.update_dependency(&Dependency {
//                name: "regex".to_string(),
//                source: DependencySource::Git {
//                    url: "http://example.com".to_string(),
//                    version: None,
//                    branch: None,
//                },
//                optional: false,
//            }).unwrap();
//        },
//    );
//
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//regex = { git = "http://example.com" }
//"#,
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//regex = { git = "http://example.com", branch = "dev" }
//"#,
//        |toml| {
//            toml.update_dependency(&Dependency {
//                name: "regex".to_string(),
//                source: DependencySource::Git {
//                    url: "http://example.com".to_string(),
//                    version: None,
//                    branch: Some("dev".to_string()),
//                },
//                optional: false,
//            }).unwrap();
//        },
//    );
//}

fn check_cargo_toml_edit(before: &str, after: &str, edit: impl FnOnce(&mut CargoToml)) {
    let mut cargo_toml = CargoToml::new(before).unwrap();
    edit(&mut cargo_toml);
    let actual = cargo_toml.text();
    assert_eq!(after, &actual);
}
