use std::iter;
use tom::{
    Factory, CstNode,
    ast,
};
use testutils::assert_eq_text;

#[test]
fn create_key_with_space() {
    check(
        |f| {
            let key = f.key("foo bar");
            key
                .cst()
        },
        "\"foo bar\"",
    );
}

#[test]
fn create_entry() {
    check(
        |f| {
            let key = f.key("foo");
            let val = f.value_string("1.0");
            f.entry(iter::once(key), val)
                .cst()
        },
        r#"foo = "1.0""#,
    );
}

fn simple_entry<'f>(f: &'f Factory, key: &str, val: &str) -> ast::Entry<'f> {
    let key = f.key(key);
    let val = f.value_string(val);
    f.entry(iter::once(key), val)
}

#[test]
fn create_dict() {
    check(
        |f| {
            let a = simple_entry(f, "foo", "1.0");
            let b = simple_entry(f, "bar", "0.0.1");
            f.value_dict(vec![a, b].into_iter())
                .cst()
        },
        r#"{ foo = "1.0", bar = "0.0.1" }"#,
    );
}

#[test]
fn create_array() {
    check(
        |f| {
            let a = f.value_number(92);
            let b = f.value_number(62);
            f.value_array(vec![a, b].into_iter())
                .cst()
        },
        "[ 92, 62 ]",
    );
}

#[test]
fn create_table() {
    check(
        |f| {
            let a = simple_entry(f, "foo", "1.0");
            let b = simple_entry(f, "bar", "0.0.1");

            f.table(
                vec![f.key("target"), f.key("x86_64.json"), f.key("dependencies")].into_iter(),
                vec![a, b].into_iter(),
            ).cst()
        },
        r#"[target."x86_64.json".dependencies]
foo = "1.0"
bar = "0.0.1""#,
    );
}

fn check(f: impl for<'f> FnOnce(&'f Factory) -> CstNode<'f>, expected: &str)
{
    let factory = Factory::new();
    let cst = f(&factory);
    assert_eq_text(
        expected,
        cst.text(),
    )
}
