extern crate tom;
use tom::{
    TomlDoc, CstNode, Symbol, TextRange,
    symbol::*,
};

fn main() {
    let text = "\
title = \"TOML Example\"

[owner]
name = \"Tom Preston-Werner\"
dob = 1979-05-27T07:32:00-08:00 # First class dates
";

    // `TomlDoc` is the API entry point.
    let doc = TomlDoc::new(text);

    // Internally, `TomlDoc` is represented as a homogeneous
    // Concrete Syntax Tree (CST) which includes whitespace and
    // comments explicitely. `.debug` method can be used to see
    // this representation:
    assert_eq!(doc.debug().trim(), r##"
DOC@[0; 112)
  ENTRY@[0; 22)
    KEY@[0; 5)
      BARE_KEY@[0; 5) "title"
    WHITESPACE@[5; 6)
    EQ@[6; 7) "="
    WHITESPACE@[7; 8)
    VALUE@[8; 22)
      BASIC_STRING@[8; 22) "\"TOML Example\""
  WHITESPACE@[22; 24)
  TABLE@[24; 111)
    TABLE_HEADER@[24; 31)
      L_BRACK@[24; 25) "["
      KEY@[25; 30)
        BARE_KEY@[25; 30) "owner"
      R_BRACK@[30; 31) "]"
    WHITESPACE@[31; 32)
    ENTRY@[32; 59)
      KEY@[32; 36)
        BARE_KEY@[32; 36) "name"
      WHITESPACE@[36; 37)
      EQ@[37; 38) "="
      WHITESPACE@[38; 39)
      VALUE@[39; 59)
        BASIC_STRING@[39; 59) "\"Tom Preston-Werner\""
    WHITESPACE@[59; 60)
    ENTRY@[60; 111)
      KEY@[60; 63)
        BARE_KEY@[60; 63) "dob"
      WHITESPACE@[63; 64)
      EQ@[64; 65) "="
      WHITESPACE@[65; 66)
      VALUE@[66; 91)
        DATE_TIME@[66; 91) "1979-05-27T07:32:00-08:00"
      WHITESPACE@[91; 92)
      COMMENT@[92; 111) "# First class dates"
  WHITESPACE@[111; 112)
"##.trim());

    // Note that `new` method does not return a `Result`.
    // This is because the library can parse even partially
    // invalid toml documents. Use `.errors` method to check
    // if file has any syntax erros;
    let invalid_toml = TomlDoc::new(":-)\nfoo=1");
    println!("{}", invalid_toml.debug().trim());
    assert_eq!(invalid_toml.debug().trim(), "");

    // To access CST, use `.cst` method.
    let root: CstNode = doc.cst();

    // A CST node has a `Symbol`, which says what kind
    // of syntactic construct this node represents, a
    // range in the text and links to children, parent,
    // and siblings.
    assert_eq!(root.symbol(&doc), DOC);
    assert_eq!(root.children(&doc).iter().count(), 4);

    let trailing_newline = root.children(&doc).last().unwrap();
    let ws_symbol: Symbol = trailing_newline.symbol(&doc);
    assert_eq!(ws_symbol, WHITESPACE);
    assert_eq!(trailing_newline.get_text(&doc), "\n");

    let title = root.children(&doc).first().unwrap();
    assert_eq!(title.symbol(&doc), ENTRY);
    assert_eq!(title.range(&doc), TextRange::from_to(0.into(), 22.into()));
    assert_eq!(title.get_text(&doc), "title = \"TOML Example\"")

    // Note that every method on `CstNode` needs a `&TomlDoc` argument.
    // This is because internally a `CstNode` is just an index, and it
    // needs the document to extract information.
}
