use symbol::*;
use {TomlFile, TomlNode};

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

    pub fn append_child(&mut self, parent: TomlNode<'f>, child: TomlNode) {
        self.append_children(parent, &mut ::std::iter::once(child))
    }

    pub fn append_children(&mut self, parent: TomlNode<'f>, children: &mut Iterator<Item=TomlNode>) {
        let mut prev_sibling = parent.children().last();
        let mut buff = String::new();
        for child in children {
            if let Some(sibling) = prev_sibling {
                buff.push_str(&compute_ws(sibling, child));
            }
            buff.push_str(child.text());
            prev_sibling = Some(child)
        }
       self.append_text(parent, buff)
    }

    pub fn append_text(&mut self, node: TomlNode<'f>, text: String) {
        self.inserted.push((node, text))
    }

    pub fn delete(&mut self, node: TomlNode<'f>) {
        self.deleted.push(node)
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

fn compute_ws(left: TomlNode, right: TomlNode) -> String {
    match (left.node().symbol(), right.node().symbol()) {
        (KEY_VAL, KEY_VAL) |
        (TABLE_HEADER, KEY_VAL) => String::from("\n"),
        (TABLE, TABLE) |
        (KEY_VAL, TABLE) => String::from("\n\n"),
        _ => String::new()
    }
}
