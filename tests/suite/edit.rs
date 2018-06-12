use tom::{
    Edit,
    ast::KeyValueOwner,
};
use {check_edit};

#[test]
fn test_swap() {
    check_edit(
        "foo = true\nbar = false",
        "bar = false\nfoo = true",
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
