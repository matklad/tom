//! FIXME: write short doc here

// extern crate tom;

// use std::iter;
// use tom_syntax::{
//     TomlDoc, CstNode, Symbol, TextRange, Position::*,
//     ast::{self, EntryOwner},
//     symbol::*,
// };

// fn main() {
//     let text = "\
// title = \"TOML Example\"

// [owner]
// name = \"Tom Preston-Werner\"
// dob = 1979-05-27T07:32:00-08:00 # First class dates
// ";

//     // `TomlDoc` is the API entry point.
//     let doc = TomlDoc::new(text);

//     // Internally, `TomlDoc` is represented as a homogeneous
//     // Concrete Syntax Tree (CST) which includes whitespace and
//     // comments explicitely. `.debug` method can be used to see
//     // this representation:
//     assert_eq!(
//         doc.debug().trim(),
//         r##"
// DOC@[0; 112)
//   ENTRY@[0; 22)
//     KEY@[0; 5)
//       BARE_KEY@[0; 5) "title"
//     WHITESPACE@[5; 6)
//     EQ@[6; 7) "="
//     WHITESPACE@[7; 8)
//     VALUE@[8; 22)
//       BASIC_STRING@[8; 22) "\"TOML Example\""
//   WHITESPACE@[22; 24)
//   TABLE@[24; 111)
//     TABLE_HEADER@[24; 31)
//       L_BRACK@[24; 25) "["
//       KEY@[25; 30)
//         BARE_KEY@[25; 30) "owner"
//       R_BRACK@[30; 31) "]"
//     WHITESPACE@[31; 32)
//     ENTRY@[32; 59)
//       KEY@[32; 36)
//         BARE_KEY@[32; 36) "name"
//       WHITESPACE@[36; 37)
//       EQ@[37; 38) "="
//       WHITESPACE@[38; 39)
//       VALUE@[39; 59)
//         BASIC_STRING@[39; 59) "\"Tom Preston-Werner\""
//     WHITESPACE@[59; 60)
//     ENTRY@[60; 111)
//       KEY@[60; 63)
//         BARE_KEY@[60; 63) "dob"
//       WHITESPACE@[63; 64)
//       EQ@[64; 65) "="
//       WHITESPACE@[65; 66)
//       VALUE@[66; 91)
//         DATE_TIME@[66; 91) "1979-05-27T07:32:00-08:00"
//       WHITESPACE@[91; 92)
//       COMMENT@[92; 111) "# First class dates"
//   WHITESPACE@[111; 112)
// "##
//             .trim()
//     );

//     // Note that `new` method does not return a `Result`.
//     // This is because the library can parse even partially
//     // invalid toml documents. Use `.errors` method to check
//     // if file has any syntax errors;
//     let invalid_toml = TomlDoc::new(":-)\nfoo=1");
//     assert_eq!(
//         invalid_toml.debug().trim(),
//         r#"
// DOC@[0; 9)
//   ERROR@[0; 1)
//     ERROR@[0; 1) ":"
//   ENTRY@[1; 9)
//     KEY@[1; 2)
//       BARE_KEY@[1; 2) "-"
//     ERROR@[2; 3)
//       ERROR@[2; 3) ")"
//     WHITESPACE@[3; 4)
//     KEY@[4; 7)
//       BARE_KEY@[4; 7) "foo"
//     EQ@[7; 8) "="
//     VALUE@[8; 9)
//       NUMBER@[8; 9) "1"

// error@[0; 1) ":": expected a key
// error@[2; 3) ")": expected `.`
// error@[1; 8) "-)\nfoo=": newlines are forbidden in entries"#.trim()
//     );
//     assert_eq!(invalid_toml.errors().len(), 3);

//     // To access CST, use `.cst` method.
//     let root: CstNode = doc.cst();

//     // A CST node has a `Symbol`, which says what kind
//     // of syntactic construct this node represents, a
//     // range in the text and links to children, parent,
//     // and siblings.
//     //
//     // Note that every method on `CstNode` needs a `&TomlDoc` argument.
//     // This is because internally a `CstNode` is just an index, and it
//     // needs the document to extract information.
//     assert_eq!(root.symbol(&doc), DOC);
//     assert_eq!(root.children(&doc).iter().count(), 4);

//     let trailing_newline = root.children(&doc).last().unwrap();
//     let ws_symbol: Symbol = trailing_newline.symbol(&doc);
//     assert_eq!(ws_symbol, WHITESPACE);
//     assert_eq!(trailing_newline.get_text(&doc), "\n");

//     let title = root.children(&doc).first().unwrap();
//     assert_eq!(title.symbol(&doc), ENTRY);
//     assert_eq!(title.range(&doc), TextRange::from_to(0.into(), 22.into()));
//     assert_eq!(title.parent(&doc), Some(root));
//     assert_eq!(title.get_text(&doc), "title = \"TOML Example\"");

//     // Working with CST directly is powerful, because you get access to
//     // all comments, punctuation and whitespace, but it is not convenient
//     // if you need the meaning of the toml document. For that purpose,
//     // a higher-level `ast` API exists.
//     let root: ast::Doc = doc.ast();
//     assert_eq!(root.entries(&doc).count(), 1);
//     assert_eq!(root.tables(&doc).count(), 1);
//     let table: ast::Table = root.tables(&doc).next().unwrap();
//     assert_eq!(table.entries(&doc).count(), 2);
//     let entry: ast::Entry = table.entries(&doc).next().unwrap();
//     match entry.value(&doc).kind(&doc) {
//         ast::ValueKind::StringLit(lit) => {
//             assert_eq!(lit.value(&doc), "Tom Preston-Werner");
//         }
//         _ => panic!("unexpected entry"),
//     }

//     // Internally, each AST node is a wrapper of the corresponding CST node,
//     // and you can use `.cst()` and `::cast()` methods to convert between the two:
//     let entry_raw = entry.cst();
//     assert_eq!(entry_raw.get_text(&doc), "name = \"Tom Preston-Werner\"");
//     match ast::Entry::cast(entry_raw, &doc) {
//         Some(entry) => assert_eq!(entry.keys(&doc).next().unwrap().name(&doc), "name"),
//         None => panic!("can't cast a node to entry"),
//     }
//     // Currently, the library does not provide a visitor API.
//     // You can build one oneself in less then a hundred lines, however.
//     // Take a look at the `visitor` and `validator` modules, which define
//     // and use a generic visitor. This API is not currently public just
//     // because the API surface is very large in comparison to small
//     // implementation.

//     // Let's see how the API for modifying the document works.
//     // First, we need to call `.start_edit` method. This is required
//     // for two reasons:
//     //   * A small edit can change the ranges of a large number of CST nodes
//     //     which makes maintaining correct ranges costly. Thus, in edit mode
//     //     ranges are assumed to be invalid and `CstNode::range` panics.
//     //   * By design, arbitrary CST structure can be created during edits.
//     //     That means that methods that assume that, for example, `ast::Value`'s
//     //     parent is always an `ast::Entry` may panic as well.
//     let mut doc = doc;
//     doc.start_edit();

//     // The primary way to create new elements is the family of
//     // `TomlDoc::new_foo_from_text` methods. Using raw text, you
//     // have full control of formatting, whitespace and comments:
//     let new_entry: ast::Entry = doc.new_entry_from_text("foo= 92 #comments are preserved");
//     assert_eq!(
//         new_entry.cst().get_text(&doc),
//         "foo= 92 #comments are preserved"
//     );

//     // If the text can not be parsed as a requested syntactic construct,
//     // the call will panic:
//     //
//     // doc.new_entry_from_text(":("); // panics!

//     // You can also create new nodes in a typed way from components,
//     // using `TomlDoc::new_foo` family of methods.
//     let new_entry: ast::Entry = {
//         let key: ast::Key = doc.new_key("foo");
//         let value: ast::Value = doc.new_value(92);
//         doc.new_entry(iter::once(key), value)
//     };
//     assert_eq!(new_entry.cst().get_text(&doc), "foo = 92");

//     // `TomlDoc` has several methods which manipulate trees:
//     // `replace`, `detach`, `insert`.
//     // Because each node is an index, and not a reference, we
//     // are able mutate document without invalidating existing
//     // nodes.
//     doc.replace(entry, new_entry);
//     assert_eq!(
//         doc.cst().get_text(&doc),
//         "\
// title = \"TOML Example\"

// [owner]
// foo = 92
// dob = 1979-05-27T07:32:00-08:00 # First class dates
// "
//     );

//     doc.detach(new_entry);
//     assert_eq!(
//         doc.cst().get_text(&doc),
//         "\
// title = \"TOML Example\"

// [owner]
// dob = 1979-05-27T07:32:00-08:00 # First class dates
// "
//     );

//     doc.insert(new_entry, PrependTo(root.cst()));
//     assert_eq!(
//         doc.cst().get_text(&doc),
//         "\
// foo = 92
// title = \"TOML Example\"

// [owner]
// dob = 1979-05-27T07:32:00-08:00 # First class dates
// "
//     );

//     // AST nodes have type-safe mutating methods as well,
//     // although currently only few are actually implemented :)
//     let e1 = doc.new_entry_from_text("foo = 1");
//     let e2 = doc.new_entry_from_text("bar = 2");

//     let d = doc.new_dict_from_text("{}");
//     assert_eq!(d.cst().get_text(&doc), "{}");

//     d.append_entry(&mut doc, e1);
//     assert_eq!(d.cst().get_text(&doc), "{ foo = 1 }");

//     d.append_entry(&mut doc, e2);
//     assert_eq!(d.cst().get_text(&doc), "{ foo = 1, bar = 2 }");

//     // Remember that we've begun the editing with `.start_edit` call?
//     // We have two ways to finish edit, with different tradeoffs.
//     //
//     // `doc.finish_edit_no_reparse();` will recalculate correct
//     // ranges for elements, but it **will not** check that CST
//     // structure is valid. However, all existing CST and AST
//     // nodes (which, as a reminder, are indices), remain valid.
//     //
//     // `doc.finish_edit_full_reparse();` will reparse the document,
//     // creating a fresh, valid CST. Because the CST structure might
//     // change after the reparse, old indices become invalid.
//     doc.finish_edit_no_reparse();
// }

fn main() {
}
