use tom::{
    Factory, CstNode,
};
use testutils::assert_eq_text;
use check_panics;

#[test]
fn create_entry_trivial() {
    check(
        |f| {
            let val = f.value_string("1.0");
            f.entry("foo", val)
                .cst()
        },
        r#"foo = "1.0""#,
    );
}

#[test]
fn create_entry_space_in_key() {
    check(
        |f| {
            let val = f.value_string("1.0");
            f.entry("foo bar", val)
                .cst()
        },
        r#""foo bar" = "1.0""#,
    );
}

#[test]
fn create_dict() {
    check(
        |f| {
            let va = f.value_string("1.0");
            let a = f.entry("foo", va);
            let vb = f.value_string("0.0.1");
            let b = f.entry("bar", vb);
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
            let va = f.value_string("1.0");
            let a = f.entry("foo", va);
            let vb = f.value_string("0.0.1");
            let b = f.entry("bar", vb);

            f.table()
                .with_names(vec!["target", "x86_64.json", "dependencies"].into_iter())
                .with_entries(vec![a, b].into_iter())
                .build()
                .cst()
        },
        r#"[target."x86_64.json".dependencies]
foo = "1.0"
bar = "0.0.1""#,
    );
}


#[test]
fn table_with_two_names() {
    covers!("table_with_two_names");
    let f = Factory::new();
    check_panics(|| {
        f.table()
            .with_name("foo")
            .with_name("bar")
            .build();
    })
}

#[test]
fn table_without_name() {
    covers!("table_without_name");
    let f = Factory::new();
    check_panics(|| {
        f.table().build();
    });
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
