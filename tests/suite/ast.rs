use tom::{ast::{self, AstNode}};
use ::{toml, find};

#[test]
fn test_find_table() {
    let toml = toml(
        r#"
[foo]
x = 1

[foo.bar]
x = 2

[foo.baz]
x = 3

['escaped key']
x = 4
        "#
    );
    let ast = toml.ast();

    check_table(ast.find_table_by_key("foo"), "[foo]\nx = 1");

    check_table(ast.find_table_by_keys(&["foo", "bar"]), "[foo.bar]\nx = 2");

    let keys = vec!["foo".to_owned(), "baz".to_owned()];
    check_table(
        ast.filter_tables(keys.iter().map(String::as_str)).next(),
        "[foo.baz]\nx = 3",
    );

    check_table(
        ast.find_table_by_key("escaped key"),
        "['escaped key']\nx = 4",
    );

    fn check_table(tbl: Option<ast::Table>, expected: &str) {
        match tbl {
            None => panic!("no table:\n{}\n", expected),
            Some(tbl) => {
                let expected = expected.trim();
                let actual = tbl.node().text().trim();
                if expected != actual {
                    panic!(
                        "wrong table:\nExpected:\n{}\n\nActual:\n{}\n",
                        expected, actual
                    );
                }
            }
        }
    }
}

#[test]
fn string_escaping_trivial() {
    let file = toml(r#"foo = "hello""#);
    let lit: ast::StringLit = find(&file);
    assert_eq!(lit.value(), "hello");

    let file = toml(r#"foo = 'hello world'"#);
    let lit: ast::StringLit = find(&file);
    assert_eq!(lit.value(), "hello world");
}

#[test]
#[ignore]
fn string_escaping_escape_sequences() {
    let file = toml(r#"foo = "hello\nworld""#);
    let lit: ast::StringLit = find(&file);
    assert_eq!(lit.value(), "hello\nworld");
}

#[test]
fn key_name() {
    let file = toml(r#"foo = false"#);
    let key: ast::Key = find(&file);
    assert_eq!(key.name(), "foo");

    let file = toml(r#"92 = false"#);
    let key: ast::Key = find(&file);
    assert_eq!(key.name(), "92");

    let file = toml(r#"'hello world' = false"#);
    let key: ast::Key = find(&file);
    assert_eq!(key.name(), "hello world");
}

#[test]
#[ignore]
fn key_name_with_escape() {
    let file = toml(r#""hello\nworld" = false"#);
    let key: ast::Key = find(&file);
    assert_eq!(key.name(), "hello\nworld");
}
