//! FIXME: write short doc here

use std::{mem};
use {
    CstNode, TomlDoc, ChunkedText,
    ast,
    parser,
    tree::InsertPos,
    symbol::*,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Position {
    After(CstNode),
    Before(CstNode),
    AppendTo(CstNode),
    PrependTo(CstNode),
}

impl TomlDoc {
    pub fn start_edit(&mut self) {
        self.edit_in_progress = true;
    }
    pub fn set_smart_ws(&mut self, smart: bool) {
        self.smart_ws = smart;
    }
    pub fn finish_edit_no_reparse(&mut self) {
        let mut data = mem::replace(&mut self.data, Vec::new());
        self.recalculate_ranges(&mut data);
        self.data = data;
        self.edit_in_progress = false;
    }
    pub fn finish_edit_full_reparse(&mut self) {
        let text = self.cst().get_text(self);
        *self = TomlDoc::new(&text);
    }

    pub fn insert(&mut self, what: impl Into<CstNode>, where_: Position) {
        self.assert_edit();
        let new_node = what.into();
        match where_ {
            Position::After(sibling) => {
                let parent = sibling.parent(self).unwrap();
                self.tree
                    .insert_child(parent.0, new_node.0, InsertPos::After(sibling.0));
            }
            Position::Before(sibling) => {
                let parent = sibling.parent(self).unwrap();
                self.tree
                    .insert_child(parent.0, new_node.0, InsertPos::Before(sibling.0));
            }
            Position::AppendTo(parent) => {
                self.tree
                    .insert_child(parent.0, new_node.0, InsertPos::Last);
            }
            Position::PrependTo(parent) => {
                self.tree
                    .insert_child(parent.0, new_node.0, InsertPos::First);
            }
        };
        if self.smart_ws {
            self.fix_ws(new_node);
        } else {
            covered_by!("basic_insertion_no_ws");
        }
    }

    pub fn replace(&mut self, what: impl Into<CstNode>, replacement: impl Into<CstNode>) {
        let what = what.into();
        let replacement = replacement.into();
        self.tree.replace(what.0, replacement.0);
    }

    pub fn detach(&mut self, what: impl Into<CstNode>) {
        let what = what.into();
        self.assert_edit();
        let siblings = (what.prev_sibling(self), what.next_sibling(&self));
        self.tree.detach(what.0);
        match siblings {
            (Some(left), Some(right)) => {
                if self.smart_ws
                    && left.symbol(self) == WHITESPACE
                    && right.symbol(self) == WHITESPACE
                {
                    self.tree.detach(right.0);
                }
            }
            _ => (),
        }
    }

    pub fn swap(&mut self, node1: impl Into<CstNode>, node2: impl Into<CstNode>) {
        let node1 = node1.into();
        let node2 = node2.into();
        let tmp = self.new_whitespace("");
        self.replace(node1, tmp);
        self.replace(node2, node1);
        self.replace(tmp, node2);
    }

    pub fn new_key(&mut self, name: &str) -> ast::Key {
        let res = self
            .new_entry_from_text(&format!("{} = 92", escaped_key(name)))
            .keys(self)
            .next()
            .unwrap();
        self.detach(res);
        res
    }

    pub fn new_value_from_text(&mut self, text: &str) -> ast::Value {
        let res = self
            .new_entry_from_text(&format!("foo = {}", text))
            .value(self);
        self.detach(res);
        res
    }

    pub fn new_value(&mut self, val: impl IntoValue) -> ast::Value {
        self.new_value_from_text(&val.value_text())
    }

    pub fn new_value_dict(&mut self, entries: impl Iterator<Item = ast::Entry>) -> ast::Value {
        let buff = join(self, entries, '{', '}');
        let res = self
            .new_entry_from_text(&format!("foo = {}", buff))
            .value(self);
        self.detach(res);
        res
    }

    pub fn new_value_array(&mut self, entries: impl Iterator<Item = ast::Value>) -> ast::Value {
        let buff = join(self, entries, '[', ']');
        let res = self
            .new_entry_from_text(&format!("foo = {}", buff))
            .value(self);
        self.detach(res);
        res
    }

    pub fn new_dict_from_text(&mut self, text: &str) -> ast::Dict {
        match self.new_value_from_text(text).kind(self) {
            ast::ValueKind::Dict(d) => d,
            _ => panic!("not a valid dict: {:?}", text),
        }
    }

    pub fn new_entry_from_text(&mut self, text: &str) -> ast::Entry {
        let entry = self.new_doc_from_text(text).entries(self).next().unwrap();
        self.detach(entry);
        entry
    }

    pub fn new_entry(
        &mut self,
        keys: impl Iterator<Item = ast::Key>,
        value: ast::Value,
    ) -> ast::Entry {
        let mut buff = String::new();
        join_to(self, &mut buff, keys, ".", "", "");
        buff.push_str(" = ");
        value.cst().chunked_text(self).write_to(&mut buff);
        self.new_entry_from_text(&buff)
    }

    pub fn new_doc_from_text(&mut self, text: &str) -> ast::Doc {
        self.assert_edit();
        let new_root = self.tree.new_internal(DOC);
        parser::parse(text, self, new_root);
        ast::Doc::cast(CstNode(new_root), self).unwrap()
    }

    pub fn new_table_from_text(&mut self, text: &str) -> ast::Table {
        let doc = self.new_doc_from_text(text);
        let res = doc.tables(self).next().unwrap();
        self.detach(res);
        res
    }

    pub fn new_table(
        &mut self,
        keys: impl Iterator<Item = ast::Key>,
        entries: impl Iterator<Item = ast::Entry>,
    ) -> ast::Table {
        let text = self.table_text(keys, entries, "[", "]");
        self.new_table_from_text(&text)
    }

    pub fn new_array_table_from_text(&mut self, text: &str) -> ast::ArrayTable {
        let doc = self.new_doc_from_text(text);
        let res = doc.array_tables(self).next().unwrap();
        self.detach(res);
        res
    }

    pub fn new_array_table(
        &mut self,
        keys: impl Iterator<Item = ast::Key>,
        entries: impl Iterator<Item = ast::Entry>,
    ) -> ast::ArrayTable {
        let text = self.table_text(keys, entries, "[[", "]]");
        self.new_array_table_from_text(&text)
    }

    pub fn new_whitespace(&mut self, ws: &str) -> CstNode {
        let idx = self.intern.intern(ws);
        CstNode(self.tree.new_leaf((WHITESPACE, idx)))
    }

    pub fn new_comma(&mut self) -> CstNode {
        let idx = self.intern.intern(",");
        CstNode(self.tree.new_leaf((COMMA, idx)))
    }

    fn assert_edit(&self) {
        assert!(
            self.edit_in_progress,
            "call .start_edit to enable editing operations"
        )
    }

    fn fix_ws(&mut self, new_node: CstNode) {
        let parent = new_node.parent(self).unwrap();

        if let Some(prev) = new_node.prev_sibling(self) {
            fix_ws_between(self, parent, prev, new_node);
        } else {
            fix_ws_before(self, parent, new_node);
        }

        if let Some(next) = new_node.next_sibling(self) {
            fix_ws_between(self, parent, new_node, next);
        } else {
            fix_ws_after(self, parent, new_node);
        }

        fn fix_ws_between(doc: &mut TomlDoc, parent: CstNode, left: CstNode, right: CstNode) {
            let ws = match (left.symbol(doc), right.symbol(doc)) {
                (ENTRY, R_CURLY) | (L_CURLY, ENTRY) | (COMMA, ENTRY) => " ",
                (ENTRY, ENTRY) | (TABLE_HEADER, ENTRY) => "\n",
                (TABLE, TABLE) | (ENTRY, TABLE) => "\n\n",
                _ => "",
            };
            if !ws.is_empty() {
                let ws = doc.new_whitespace(ws);
                doc.tree
                    .insert_child(parent.0, ws.0, InsertPos::After(left.0));
            }
        }

        fn fix_ws_before(_doc: &mut TomlDoc, _parent: CstNode, _last_child: CstNode) {}

        fn fix_ws_after(doc: &mut TomlDoc, parent: CstNode, last_child: CstNode) {
            let ws = if parent.symbol(doc) == DOC && last_child.symbol(doc) != WHITESPACE {
                "\n"
            } else {
                ""
            };
            if !ws.is_empty() {
                let ws = doc.new_whitespace(ws);
                doc.tree
                    .insert_child(parent.0, ws.0, InsertPos::After(last_child.0));
            }
        }
    }

    fn table_text(
        &mut self,
        keys: impl Iterator<Item = ast::Key>,
        entries: impl Iterator<Item = ast::Entry>,
        left: &str,
        right: &str,
    ) -> String {
        let mut buff = String::new();
        buff.push_str(left);
        join_to(self, &mut buff, keys, ".", "", "");
        buff.push_str(right);
        for entry in entries {
            buff.push('\n');
            entry.cst().chunked_text(self).write_to(&mut buff);
        }
        buff
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for bool {}

    impl Sealed for i64 {}

    impl<'a> Sealed for &'a str {}
}

pub trait IntoValue: private::Sealed {
    #[doc(hidden)]
    fn value_text(self) -> String;
}

impl IntoValue for bool {
    fn value_text(self) -> String {
        if self { "true" } else { "false" }.to_owned()
    }
}

impl IntoValue for i64 {
    fn value_text(self) -> String {
        self.to_string()
    }
}

impl<'a> IntoValue for &'a str {
    fn value_text(self) -> String {
        //FIXME: escaping
        format!("{:?}", self)
    }
}

fn join<A: Into<CstNode>>(
    doc: &TomlDoc,
    items: impl Iterator<Item = A>,
    left: char,
    right: char,
) -> String {
    let mut buff = String::new();
    buff.push(left);
    join_to(doc, &mut buff, items, ", ", " ", " ");
    buff.push(right);

    buff
}

fn join_to<A: Into<CstNode>>(
    doc: &TomlDoc,
    buff: &mut String,
    items: impl Iterator<Item = A>,
    sep: &str,
    before_first: &str,
    after_last: &str,
) {
    let mut first = true;
    for item in items {
        if first {
            buff.push_str(before_first);
        }

        if !first {
            buff.push_str(sep);
        }
        first = false;
        item.into().chunked_text(doc).write_to(buff);
    }
    if !first {
        buff.push_str(after_last);
    }
}

fn escaped_key(key: &str) -> String {
    if key
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        key.to_string()
    } else {
        format!("{:?}", key)
    }
}
