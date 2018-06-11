use tom::{TomlFile, ast::{self, AstNode}};

#[test]
fn test_find_table() {
    let toml = TomlFile::new(
        r#"
[foo]
x = 1

[foo.bar]
x = 2

[foo.baz]
x = 3

['escaped key']
x = 4
        "#.to_owned(),
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
