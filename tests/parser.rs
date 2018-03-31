extern crate tom;
extern crate testutils;
extern crate parse_tree;

use parse_tree::debug_dump;


#[test]
fn smoke_test() {
    let text = "\
foo = 92
bar = false
";
    let tree = tom::parse(text);
    let dump = debug_dump(tree.root(), text);
    assert_eq!(
        dump.trim(), r#"
FILE@[0; 20)
  KEY_VAL@[0; 8)
    KEY@[0; 3)
      BARE_KEY@[0; 3) "foo"
    WHITESPACE@[3; 4)
    EQ@[4; 5) "="
    VAL@[5; 8)
      WHITESPACE@[5; 6)
      NUMBER@[6; 8) "92"
  KEY_VAL@[8; 20)
    KEY@[8; 12)
      WHITESPACE@[8; 9)
      BARE_KEY@[9; 12) "bar"
    WHITESPACE@[12; 13)
    EQ@[13; 14) "="
    VAL@[14; 20)
      WHITESPACE@[14; 15)
      BOOL@[15; 20) "false"
"#.trim());
}
