use TomlFile;
use edit::TextEdit;
use ast::{self, AstNode, TableHeaderOwner, KeyValueOwner, Table, KeyVal, File, Val};
use parse_tree::{TextUnit, Node};
use symbols::WHITESPACE;


/// Sorts entries in the `[dependencies]` section of Cargo.toml
pub fn sort_dependencies(toml: &TomlFile) -> Option<TextEdit> {
    let deps_table = toml.ast().find_table("dependencies")?;
    let mut entries: Vec<_> = deps_table.entries().collect();
    entries.sort_by_key(|e| e.key().node().text());

    let mut edit = toml.edit();
    for (old, new) in deps_table.entries().zip(entries) {
        edit.replace(old.node(), new.node());
    }
    Some(edit.into_text_edit())
}

fn dependencies_table(toml: &TomlFile) -> Option<Table> {
    toml.ast().find_table("dependencies")
}

#[test]
fn test_sort_dependencies() {
    let before = r#"
[package]
name = "tom"
version = "0.1.0"
authors = ["Aleksey Kladov <aleksey.kladov@gmail.com>"]

[dependencies]
regex= "0.2"
lalrpop-util = "0.15"
parse_tree = {  path = "../parse_tree"}
"#;

    let after = r#"
[package]
name = "tom"
version = "0.1.0"
authors = ["Aleksey Kladov <aleksey.kladov@gmail.com>"]

[dependencies]
lalrpop-util = "0.15"
parse_tree = {  path = "../parse_tree"}
regex= "0.2"
"#;

    let toml = ::TomlFile::new(before.to_string());
    let edit = sort_dependencies(&toml).unwrap();
    let actual = edit.apply(before);
    assert_eq!(after, actual);
}

pub fn add_dependency(file: &TomlFile, name: &str, version: &str) -> Option<TextEdit> {
    let deps_table = dependencies_table(file)?;
    let mut edit = file.edit();
    edit.insert_text_after(
        deps_table.node(),
        format!("\n{} = \"{}\"", name, version),
    );
    Some(edit.into_text_edit())
}

#[test]
fn test_add_dependency() {
    let before = r#"
[package]
name = "tom"
version = "0.1.0"
authors = ["Aleksey Kladov <aleksey.kladov@gmail.com>"]

[dependencies]
regex= "0.2"
parse_tree = {  path = "../parse_tree"}

[dev-dependencies]
lalrpop = "0.15"
"#;

    let after = r#"
[package]
name = "tom"
version = "0.1.0"
authors = ["Aleksey Kladov <aleksey.kladov@gmail.com>"]

[dependencies]
regex= "0.2"
parse_tree = {  path = "../parse_tree"}
lalrpop-util = "0.15"

[dev-dependencies]
lalrpop = "0.15"
"#;

    let toml = ::TomlFile::new(before.to_string());
    let edit = add_dependency(&toml, "lalrpop-util", "0.15").unwrap();
    let actual = edit.apply(before);
    assert_eq!(after, actual);
}


pub fn dict_to_table(toml: &TomlFile, offset: TextUnit) -> Option<TextEdit> {
    let key_val: KeyVal = ast::search::node_at_offset(toml.ast().node(), offset)?;
    let parent = key_val.node().parent().unwrap();
    let dict = match key_val.val() {
        Val::Dict(dict) => dict,
        _ => return None
    };

    let (table_prefix, table) = if let Some(_) = File::cast(parent) {
        ("".to_string(), None)
    } else if let Some(table) = Table::cast(parent) {
        let prefix = table.header().keys()
            .map(|it| format!("{}.", it.node().text()))
            .collect();
        (prefix, Some(table))
    } else {
        return None;
    };

    let mut text = format!("[{}{}]\n", table_prefix, key_val.key().node().text());
    for entry in dict.entries() {
        text += entry.node().text();
        text += "\n";
    }

    let mut edit = toml.edit();
    match table {
        None => edit.replace_with_text(key_val.node(), text),
        Some(table) => {
            edit.delete(key_val.node());
            if let Some(ws) = prev_sibling(key_val.node()) {
                if ws.symbol() == WHITESPACE {
                    edit.delete(ws);
                }
            }
            edit.insert_text_after(
                table.node(),
                format!("\n{}", text),
            );
        }
    }
    Some(edit.into_text_edit())
}

#[test]
pub fn test_dict_to_table() {
    do_test(r#"
package = { name = "foo", version = "92" }
        "#, r#"
[package]
name = "foo"
version = "92"
        "#,
            10);

    do_test(r#"
[dependencies]
foo = { git = "http://example.com", branch = "dev" }
bar = "1.0"
        "#, r#"
[dependencies]
bar = "1.0"
[dependencies.foo]
git = "http://example.com"
branch = "dev"
        "#,
            20);


    fn do_test(before: &str, after: &str, offset: u32) {
        let before = before.trim();
        let after = after.trim();
        let toml = ::TomlFile::new(before.to_string());
        let edit = dict_to_table(&toml, offset.into()).unwrap();
        let actual = edit.apply(before);
        assert_eq!(after, actual.trim());
    }
}


fn prev_sibling<'t>(node: Node<'t>) -> Option<Node<'t>> {
    let parent = node.parent()?;
    let ch = parent.children();
    for (c1, c2) in parent.children().zip(ch.skip(1)) {
        if c2 == node {
            return Some(c1);
        }
    }
    None
}
