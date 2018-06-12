use tom::ast;
use ::{toml, find};

#[test]
fn string_escaping_trivial() {
    let doc = toml(r#"foo = "hello""#);
    let lit: ast::StringLit = find(&doc);
    assert_eq!(lit.value(), "hello");

    let doc = toml(r#"foo = 'hello world'"#);
    let lit: ast::StringLit = find(&doc);
    assert_eq!(lit.value(), "hello world");
}

#[test]
#[ignore]
fn string_escaping_escape_sequences() {
    let doc = toml(r#"foo = "hello\nworld""#);
    let lit: ast::StringLit = find(&doc);
    assert_eq!(lit.value(), "hello\nworld");
}

#[test]
fn key_name() {
    let doc = toml(r#"foo = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(), "foo");

    let doc = toml(r#"92 = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(), "92");

    let doc = toml(r#"'hello world' = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(), "hello world");
}

#[test]
#[ignore]
fn key_name_with_escape() {
    let doc = toml(r#""hello\nworld" = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(), "hello\nworld");
}
