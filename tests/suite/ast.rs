use tom::ast;
use ::{find, toml};

#[test]
fn string_escaping_trivial() {
    let doc = toml(r#"foo = "hello""#);
    let lit: ast::StringLit = find(&doc);
    assert_eq!(lit.value(&doc), "hello");

    let doc = toml(r#"foo = 'hello world'"#);
    let lit: ast::StringLit = find(&doc);
    assert_eq!(lit.value(&doc), "hello world");
}

#[test]
fn bool_value() {
    let doc = toml(r"foo = true");
    let lit: ast::Bool = find(&doc);
    assert_eq!(lit.value(&doc), true);

    let doc = toml(r"foo = false");
    let lit: ast::Bool = find(&doc);
    assert_eq!(lit.value(&doc), false);
}

#[test]
#[ignore]
fn int_value() {
    let doc = toml(r"foo = 92");
    let lit: ast::Integer = find(&doc);
    assert_eq!(lit.value(&doc), 92);
}

#[test]
#[ignore]
fn date_time_value() {
    let doc = toml(r"foo = 1979-05-27T00:32:00.999999-07:00");
    let lit: ast::DateTime = find(&doc);
    assert_eq!(lit.value(&doc), ::std::time::UNIX_EPOCH);
}

#[test]
#[ignore]
fn string_escaping_escape_sequences() {
    let doc = toml(r#"foo = "hello\nworld""#);
    let lit: ast::StringLit = find(&doc);
    assert_eq!(lit.value(&doc), "hello\nworld");
}

#[test]
fn key_name() {
    let doc = toml(r#"foo = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(&doc), "foo");

    let doc = toml(r#"92 = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(&doc), "92");

    let doc = toml(r#"'hello world' = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(&doc), "hello world");
}

#[test]
#[ignore]
fn key_name_with_escape() {
    let doc = toml(r#""hello\nworld" = false"#);
    let key: ast::Key = find(&doc);
    assert_eq!(key.name(&doc), "hello\nworld");
}
