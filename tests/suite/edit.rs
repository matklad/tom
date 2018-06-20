use tom::{
    Edit, Factory,
    ast,
};
use {toml, check_edit, check_panics};
use tom::Position;
use tom::TomlDoc;


#[test]
fn basic_insertion() {
    let before = r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#;

    let f = Factory::new();
    let quux = f.entry("quux", f.val_string("92"));

    check_edit(
        before,
        r#"
foo = "1.0.0"
quux = "92"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc| {
            let mut edit = Edit::new(doc);
            let (foo, _, _) = foobarbaz(doc);
            edit.insert(quux, Position::after(foo));
            edit.finish()
        },
    );

    check_edit(
        before,
        r#"
foo = "1.0.0"

quux = "92"
bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc| {
            let mut edit = Edit::new(doc);
            let (_, bar, _) = foobarbaz(doc);
            edit.insert(quux, Position::before(bar));
            edit.finish()
        },
    );

    check_edit(
        before,
        r#"quux = "92"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc| {
            let mut edit = Edit::new(doc);
            edit.insert(quux, Position::start_of(doc.ast()));
            edit.finish()
        },
    );

    check_edit(
        before,
        r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
quux = "92"
"#,
        |doc| {
            let mut edit = Edit::new(doc);
            edit.insert(quux, Position::end_of(doc.ast()));
            edit.finish()
        },
    );

    fn foobarbaz(doc: &TomlDoc) -> (ast::Entry, ast::Entry, ast::Entry) {
        let entries: Vec<_> = doc.ast().entries().collect();
        (entries[0], entries[1], entries[2])
    }
}

#[test]
fn basic_insertion_no_ws() {
    let before = r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#;

    let f = Factory::new();
    let quux = f.entry("quux", f.val_string("92"));

    check_edit(
        before,
        r#"
foo = "1.0.0"quux = "92"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc| {
            let mut edit = Edit::new(doc);
            edit.disable_smart_whitespace();
            let (foo, _, _) = foobarbaz(doc);
            edit.insert(quux, Position::after(foo));
            edit.finish()
        },
    );

    check_edit(
        before,
        r#"
foo = "1.0.0"

quux = "92"bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc| {
            let mut edit = Edit::new(doc);
            edit.disable_smart_whitespace();
            let (_, bar, _) = foobarbaz(doc);
            edit.insert(quux, Position::before(bar));
            edit.finish()
        },
    );

    check_edit(
        before,
        r#"quux = "92"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
"#,
        |doc| {
            let mut edit = Edit::new(doc);
            edit.disable_smart_whitespace();
            edit.insert(quux, Position::start_of(doc.ast()));
            edit.finish()
        },
    );

    check_edit(
        before,
        r#"
foo = "1.0.0"

bar = "1.0.0"

baz = "1.0.0"
quux = "92""#,
        |doc| {
            let mut edit = Edit::new(doc);
            edit.disable_smart_whitespace();
            edit.insert(quux, Position::end_of(doc.ast()));
            edit.finish()
        },
    );

    fn foobarbaz(doc: &TomlDoc) -> (ast::Entry, ast::Entry, ast::Entry) {
        let entries: Vec<_> = doc.ast().entries().collect();
        (entries[0], entries[1], entries[2])
    }
}


#[test]
fn basic_deletion() {
    check_edit(
        "foo = true\nbar = false\nbaz = false\n",
        "foo = true\n\nbaz = false\n",
        |doc| {
            let mut edit = Edit::new(doc);
            let ast = doc.ast();
            let bar = ast.entries().nth(1).unwrap();
            edit.delete(bar);
            edit.finish()
        }
    )
}

#[test]
fn test_swap() {
    check_edit(
        "foo = true\nbar = false\n",
        "bar = false\nfoo = true\n",
        |doc| {
            let mut edit = Edit::new(doc);
            let ast = doc.ast();
            let foo = ast.entries().nth(0).unwrap();
            let bar = ast.entries().nth(1).unwrap();
            edit.replace(foo, bar);
            edit.replace(bar, foo);
            edit.finish()
        },
    );
}

#[test]
fn infinite_doc() {
    covers!("infinite_doc");
    let doc = toml("foo = false");
    let entry = doc.ast().entries().next().unwrap();
    let mut edit = Edit::new(&doc);
    edit.insert(entry, Position::end_of(entry));
    check_panics(|| drop(edit.finish()));
}
