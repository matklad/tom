use tom::Factory;
use tom::ast::AstNode;

#[test]
fn test_create_key_val_trivial() {
    let f = Factory::new();
    let val = f.val_string("1.0");
    let kv = f.key_val("foo", val);
    assert_eq!(kv.node().text(), r#"foo = "1.0""#);
}

#[test]
fn test_create_key_val_space_in_key() {
    let f = Factory::new();
    let val = f.val_string("1.0");
    let kv = f.key_val("foo bar", val);
    assert_eq!(kv.node().text(), r#""foo bar" = "1.0""#);
}


#[test]
fn test_create_table() {
    let f = Factory::new();
    let va = f.val_string("1.0");
    let a = f.key_val("foo", va);
    let vb = f.val_string("0.0.1");
    let b = f.key_val("bar", vb);
    let table = f.table(
        &mut vec!["target", "x86_64.json", "dependencies"].into_iter(),
        &mut vec![a, b].into_iter(),
    );
    assert_eq!(
        table.node().text(),
        r#"
[target.'x86_64.json'.dependencies]
foo = "1.0"
bar = "0.0.1"
"#.trim()
    );
}
