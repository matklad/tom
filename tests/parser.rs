extern crate tom;
extern crate testutils;
extern crate parse_tree;

use parse_tree::debug_dump;

#[test]
fn test_ok() {
    testutils::dir_tests(
        &["ok"],
        |text| {
            let tree = tom::parse(text);
            debug_dump(tree.root(), text)
        },
    )
}
