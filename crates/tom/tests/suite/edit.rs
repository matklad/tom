use tom::{ast, Position::*, TomlDoc, TextRange};
use util::assert_eq_text;
use ::{check_edit, toml};

#[test]
fn debug_dump_works_during_edit() {
    let mut toml = toml(
        r#"
foo = "1.0.0"
"#,
    );
    toml.start_edit();
    let v = toml.new_value(92);
    let root = toml.cst();
    toml.insert(v, AppendTo(root));
    assert_eq_text(
        r#"
*modified*
DOC@[0; 18)
  WHITESPACE@[0; 1)
  ENTRY@[1; 14)
    KEY@[1; 4)
      BARE_KEY@[1; 4) "foo"
    WHITESPACE@[4; 5)
    EQ@[5; 6) "="
    WHITESPACE@[6; 7)
    VALUE@[7; 14)
      BASIC_STRING@[7; 14) "\"1.0.0\""
  WHITESPACE@[14; 15)
  VALUE@[15; 17)
    NUMBER@[15; 17) "92"
  WHITESPACE@[17; 18)"#.trim(),
        toml.debug().trim(),
    );
}

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
        "foo = true\nbaz = false\n",
        |doc| {
            let ast = doc.ast();
            let bar = ast.entries(doc).nth(1).unwrap();
            doc.detach(bar);
        },
    )
}

#[test]
fn basic_deletion_no_ws() {
    check_edit(
        "foo = true\nbar = false\nbaz = false\n",
        "foo = true\n\nbaz = false\n",
        |doc| {
            doc.set_smart_ws(false);
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

#[test]
fn recalculates_ranges() {
    let mut doc = TomlDoc::new("foo = 92");
    let foo = doc.ast().entries(&doc).next().unwrap();
    assert_eq!(
        foo.cst().range(&doc),
        TextRange::from_to(0.into(), 8.into())
    );

    doc.start_edit();
    let bar = doc.new_entry_from_text("bar = 62");
    let root = doc.cst();
    doc.insert(bar, PrependTo(root));
    doc.finish_edit_no_reparse();
    assert_eq!(
        foo.cst().range(&doc),
        TextRange::from_to(9.into(), 17.into())
    );
}

#[test]
fn edit_full_reparse() {
    let mut doc = TomlDoc::new("");
    assert!(doc.errors().is_empty());

    doc.start_edit();
    let val = doc.new_value_from_text("92");
    let root = doc.cst();
    doc.insert(val, PrependTo(root));
    doc.finish_edit_no_reparse();
    assert!(doc.errors().is_empty());

    doc.start_edit();
    doc.finish_edit_full_reparse();
    assert!(!doc.errors().is_empty());
}
