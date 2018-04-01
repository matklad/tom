use parse_tree::{
    search::find_covering_node,
    TextRange, TextUnit, ParseTree, Node,
};
use super::text_edit::{TextEdit, TextEditBuilder, TextEditOp};

pub struct TreeEdit<'t> {
    tree: &'t ParseTree,
    inserted: Vec<(Node<'t>, String)>,
    replaced: Vec<(Node<'t>, String)>,
    deleted: Vec<Node<'t>>,
}

impl<'t> TreeEdit<'t> {
    pub fn new(tree: &'t ParseTree) -> TreeEdit {
        TreeEdit {
            tree,
            inserted: Vec::new(),
            replaced: Vec::new(),
            deleted: Vec::new(),
        }
    }

    pub fn replace(&mut self, node: Node<'t>, replacement: Node) {
        self.replace_with_text(node, replacement.text().to_string())
    }

    pub fn replace_with_text(&mut self, node: Node<'t>, replacement: String) {
        self.replaced.push((node, replacement))
    }

    pub fn replace_substring(&mut self, range: TextRange, replacement: String) {
        let root = self.tree.root();
//        assert!(is_subrange_of(root.range(), range));
        let node = find_covering_node(root, range);
        let all_text = root.text();
        let prefix = &all_text[TextRange::from_to(node.range().start(), range.start())];
        let suffix = &all_text[TextRange::from_to(range.end(), node.range().end())];
        let new_text = prefix.to_string()
            + &replacement
            + &suffix;
        self.replace_with_text(node, new_text);
    }

    pub fn delete(&mut self, node: Node<'t>) {
        self.deleted.push(node)
    }

    pub fn insert_text_after(&mut self, anchor: Node<'t>, text: String) {
        self.inserted.push((anchor, text))
    }

    pub fn into_text_edit(self) -> TextEdit {
        let root = self.tree.root();
        let mut edit_builder = TextEditBuilder::new(root.text());
        self.text_edit_for_node(root, &mut edit_builder);
        // TODO: :(
        let mut edit = edit_builder.build();
        edit.ops.push(TextEditOp::Copy(TextRange::from_len(
            (root.text().len() as u32).into(),
            0.into(),
        )));
        edit
    }

    fn text_edit_for_node(&self, node: Node<'t>, edit_builder: &mut TextEditBuilder) {
        if self.deleted.iter().find(|&&n| n == node).is_some() {
            edit_builder.delete(node.range());
            return;
        }

        if let Some(&(_, ref replacement)) = self.replaced.iter().find(|&&(n, _)| n == node) {
            edit_builder.replace(node.range(), replacement.to_string());
            return;
        }

        for child in node.children() {
            self.text_edit_for_node(child, edit_builder);
        }

        if let Some(&(_, ref replacement)) = self.inserted.iter().find(|&&(n, _)| n == node) {
            edit_builder.insert(node.range().end(), replacement.to_string());
        }
    }
}
