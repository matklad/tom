use tom::{
    Factory, TomlNode,
    ast::AstNode,
};
use testutils::assert_eq_text;
use check_panics;

#[test]
fn test_create_key_val_trivial() {
    check(
        |f| {
            let val = f.val_string("1.0");
            f.key_val("foo", val)
                .node()
        },
        r#"foo = "1.0""#,
    );
}

#[test]
fn test_create_key_val_space_in_key() {
    check(
        |f| {
            let val = f.val_string("1.0");
            f.key_val("foo bar", val)
                .node()
        },
        r#""foo bar" = "1.0""#,
    );
}

#[test]
fn create_table() {
    check(
        |f| {
            let va = f.val_string("1.0");
            let a = f.key_val("foo", va);
            let vb = f.val_string("0.0.1");
            let b = f.key_val("bar", vb);

            f.table()
                .with_names(vec!["target", "x86_64.json", "dependencies"].into_iter())
                .with_entries(vec![a, b].into_iter())
                .build()
                .node()
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

fn check(f: impl for<'f> FnOnce(&'f Factory) -> TomlNode<'f>, expected: &str)
{
    let factory = Factory::new();
    let ast = f(&factory);
    assert_eq_text(
        expected,
        ast.text(),
    )
}
