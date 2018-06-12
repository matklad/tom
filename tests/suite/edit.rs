use tom::{
    Edit,
    ast::KeyValueOwner,
};
use {toml, check_edit, check_panics};
use tom::Position;

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
        }
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
