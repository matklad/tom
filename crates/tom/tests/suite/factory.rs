use std::iter;
use util::assert_eq_text;
use tom::{CstNode, TomlDoc};

#[test]
fn create_key_with_space() {
    check(|doc| doc.new_key("foo bar"), "\"foo bar\"");
}

#[test]
fn create_entry() {
    check(
        |doc| {
            let key = doc.new_key("foo");
            let val = doc.new_value("1.0");
            doc.new_entry(iter::once(key), val)
        },
        r#"foo = "1.0""#,
    );
}

#[test]
fn create_dict() {
    check(
        |doc| {
            let a = doc.new_entry_from_text("foo = \"1.0\"");
            let b = doc.new_entry_from_text("bar = \"0.0.1\"");
            doc.new_value_dict(vec![a, b].into_iter())
        },
        r#"{ foo = "1.0", bar = "0.0.1" }"#,
    );
}

#[test]
fn create_array() {
    check(
        |doc| {
            let a = doc.new_value(92);
            let b = doc.new_value(62);
            doc.new_value_array(vec![a, b].into_iter())
        },
        "[ 92, 62 ]",
    );
}

#[test]
fn create_table() {
    check(
        |doc| {
            let a = doc.new_entry_from_text("foo = \"1.0\"");
            let b = doc.new_entry_from_text("bar = \"0.0.1\"");
            let mut keys = Vec::new();
            for key in "target x86_64.json dependencies".split_whitespace() {
                keys.push(doc.new_key(key));
            }
            doc.new_table(keys.into_iter(), vec![a, b].into_iter())
        },
        r#"[target."x86_64.json".dependencies]
foo = "1.0"
bar = "0.0.1""#,
    );
}

#[test]
fn create_array_table() {
    check(
        |doc| {
            let a = doc.new_entry_from_text("name = \"foo\"");
            let key = doc.new_key("bin");

            doc.new_array_table(iter::once(key), iter::once(a)).cst()
        },
        r#"[[bin]]
name = "foo""#,
    );
}

fn check<F: FnOnce(&mut TomlDoc) -> R, R: Into<CstNode>>(f: F, expected: &str) {
    let mut doc = TomlDoc::new("");
    doc.start_edit();
    let cst = f(&mut doc).into();
    let actual = cst.get_text(&doc);
    assert_eq_text(expected, &actual)
}
