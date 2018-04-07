use {TomlFile, TomlNode};
use ast::{self, AstNode};

#[derive(Debug)]
pub struct Edit<'f> {
    file: &'f TomlFile,
    inserted: Vec<(TomlNode<'f>, String)>,
    replaced: Vec<(TomlNode<'f>, String)>,
    deleted: Vec<TomlNode<'f>>,
}

impl<'f> Edit<'f> {
    pub fn new(file: &'f TomlFile) -> Edit {
        Edit {
            file,
            inserted: Vec::new(),
            replaced: Vec::new(),
            deleted: Vec::new(),
        }
    }

    pub fn replace(&mut self, node: TomlNode<'f>, replacement: TomlNode) {
        self.replace_with_text(node, replacement.text().to_string())
    }

    pub fn replace_with_text(&mut self, node: TomlNode<'f>, replacement: String) {
        self.replaced.push((node, replacement))
    }

    pub fn delete(&mut self, node: TomlNode<'f>) {
        self.deleted.push(node)
    }

    pub fn insert_after(&mut self, anchor: TomlNode<'f>, new_node: TomlNode) {
        let text = format!("\n{}", new_node.text());
        self.insert_text_after(anchor, text)
    }

    pub fn insert_text_after(&mut self, anchor: TomlNode<'f>, text: String) {
        self.inserted.push((anchor, text))
    }

    pub fn append_key_value(&mut self, table: ast::Table<'f>, key: &str, value: &str) {
        let last_child = table.node().children().last().unwrap();
        self.insert_text_after(last_child, format!("\n{} = {}", key, value))
    }

    pub fn finish(self) -> String {
        let mut buff = String::new();
        self.write(self.file.parse_tree(), &mut buff);
        buff
    }

    fn write(&self, node: TomlNode, buff: &mut String) {
        if self.deleted.iter().find(|&&n| n == node).is_some() {
            return;
        }
        if let Some(&(_, ref text)) = self.replaced.iter().find(|&&(n, _)| n == node) {
            buff.push_str(text);
            return;
        }

        if node.is_leaf() {
            buff.push_str(node.text())
        } else {
            for child in node.children() {
                self.write(child, buff);
            }
        }
        if let Some(&(_, ref text)) = self.inserted.iter().find(|&&(n, _)| n == node) {
            buff.push_str(text)
        }
    }
}
