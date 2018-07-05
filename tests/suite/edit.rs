use check_edit;
use tom::{ast, Position::*, TomlDoc};

#[test]
fn basic_insertion() {
    do_check(
        r#"
foo = "1.0.0"
quux = "92"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc, quux, (foo, _, _)| doc.insert(quux, After(foo.into())),
    );

    do_check(
        r#"
foo = "1.0.0"

quux = "92"
bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc, quux, (_, bar, _)| doc.insert(quux, Before(bar.into())),
    );

    do_check(
        r#"quux = "92"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc, quux, _| {
            let root = doc.ast();
            doc.insert(quux, PrependTo(root.into()))
        },
    );

    do_check(
        r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
quux = "92"
"#,
        |doc, quux, _| {
            let root = doc.ast();
            doc.insert(quux, AppendTo(root.into()));
        },
    );

    fn do_check(
        after: &str,
        f: impl Fn(&mut TomlDoc, ast::Entry, (ast::Entry, ast::Entry, ast::Entry)),
    ) {
        let before = r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#;
        check_edit(before, after, |doc| {
            let quux = doc.new_entry_from_text("quux = \"92\"");
            let entries: Vec<_> = doc.ast().entries(doc).collect();
            f(doc, quux, (entries[0], entries[1], entries[2]));
        })
    }
}

#[test]
fn basic_insertion_no_ws() {
    covers!("basic_insertion_no_ws");
    do_check(
        r#"
foo = "1.0.0"quux = "92"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc, quux, (foo, _, _)| doc.insert(quux, After(foo.into())),
    );

    do_check(
        r#"
foo = "1.0.0"

quux = "92"bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc, quux, (_, bar, _)| doc.insert(quux, Before(bar.into())),
    );

    do_check(
        r#"quux = "92"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc, quux, _| {
            let root = doc.cst();
            doc.insert(quux, PrependTo(root));
        },
    );

    do_check(
        r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
quux = "92""#,
        |doc, quux, _| {
            let root = doc.cst();
            doc.insert(quux, AppendTo(root));
        },
    );

    fn do_check(
        after: &str,
        f: impl Fn(&mut TomlDoc, ast::Entry, (ast::Entry, ast::Entry, ast::Entry)),
    ) {
        let before = r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#;
        check_edit(before, after, |doc| {
            doc.set_smart_ws(false);
            let quux = doc.new_entry_from_text("quux = \"92\"");
            let entries: Vec<_> = doc.ast().entries(doc).collect();
            f(doc, quux, (entries[0], entries[1], entries[2]));
        })
    }
}

#[test]
fn basic_deletion() {
    check_edit(
        "foo = true\nbar = false\nbaz = false\n",
        "foo = true\n\nbaz = false\n",
        |doc| {
            let ast = doc.ast();
            let bar = ast.entries(doc).nth(1).unwrap();
            doc.detach(bar);
        },
    )
}

#[test]
fn test_swap() {
    check_edit(
        "foo = true\nbar = false\n",
        "bar = false\nfoo = true\n",
        |doc| {
            let ast = doc.ast();
            let foo = ast.entries(doc).nth(0).unwrap();
            let bar = ast.entries(doc).nth(1).unwrap();
            doc.swap(foo, bar);
        },
    );
}
