extern crate testutils;
extern crate tom;

mod ast;
mod edit;

#[test]
fn test_parser_ok() {
    testutils::dir_tests(&["ok"], |text| {
        let tree = tom::TomlFile::new(text.to_string());
        tree.debug_dump()
    })
}
