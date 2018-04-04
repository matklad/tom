extern crate tom;
extern crate testutils;

#[test]
fn test_ok() {
    testutils::dir_tests(
        &["ok"],
        |text| {
            let tree = tom::TomlFile::new(text.to_string());
            tree.debug_dump()
        },
    )
}
