use {TomlDoc, CstNode, tree::InsertPos, ast, symbol::*, parser};

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
    pub fn finish_edit(&mut self) {
        unimplemented!()
    }
    fn assert_edit(&self) {
        assert!(self.edit_in_progress, "call .start_edit to enable editing operations")
    }

    pub fn insert(&mut self, what: impl Into<CstNode>, where_: Position) {
        self.assert_edit();
        let new_node = what.into();
        match where_ {
            Position::After(sibling) => {
                let parent = sibling.parent(self).unwrap();
                self.tree.tree.insert_child(parent.0, new_node.0, InsertPos::After(sibling.0));
            }
            Position::Before(sibling) => {
                let parent = sibling.parent(self).unwrap();
                self.tree.tree.insert_child(parent.0, new_node.0, InsertPos::Before(sibling.0));
            }
            Position::AppendTo(parent) => {
                self.tree.tree.insert_child(parent.0, new_node.0, InsertPos::Last);
            }
            Position::PrependTo(parent) => {
                self.tree.tree.insert_child(parent.0, new_node.0, InsertPos::First);
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
        self.tree.tree.replace(what.0, replacement.0);
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
                (ENTRY, ENTRY) | (TABLE_HEADER, ENTRY) => "\n",
                (TABLE, TABLE) | (ENTRY, TABLE) => "\n\n",
                _ => "",
            };
            if !ws.is_empty() {
                let ws = doc.new_ws(ws);
                doc.tree.tree.insert_child(parent.0, ws.0, InsertPos::After(left.0));
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
                let ws = doc.new_ws(ws);
                doc.tree.tree.insert_child(parent.0, ws.0, InsertPos::After(last_child.0));
            }
        }
    }

    pub fn detach(&mut self, what: impl Into<CstNode>) {
        self.assert_edit();
        self.tree.tree.detach(what.into().0);
    }

    pub fn swap(&mut self, node1: impl Into<CstNode>, node2: impl Into<CstNode>) {
        let node1 = node1.into();
        let node2 = node2.into();
        let tmp = self.new_ws("");
        self.replace(node1, tmp);
        self.replace(node2, node1);
        self.replace(tmp, node2);
    }


    pub fn new_key(&mut self, name: &str) -> ast::Key {
        let res = self.new_entry_from_text(&format!("{} = 92", escaped_key(name)))
            .keys(self).next().unwrap();
        self.detach(res);
        res
    }

    pub fn new_value_string(&mut self, val: &str) -> ast::Value {
        //TODO: escaping
        let res = self.new_entry_from_text(&format!("foo = {:?}", val)).value(self);
        self.detach(res);
        res
    }

    pub fn new_value_number(&mut self, val: i64) -> ast::Value {
        //TODO: escaping
        let res = self.new_entry_from_text(&format!("foo = {}", val)).value(self);
        self.detach(res);
        res
    }

    pub fn new_value_bool(&mut self, val: bool) -> ast::Value {
        let res = self.new_entry_from_text(&format!("foo = {}", val)).value(self);
        self.detach(res);
        res
    }

    pub fn new_value_dict(&mut self, entries: impl Iterator<Item=ast::Entry>) -> ast::Value {
        let buff = join(self, entries, '{', '}');
        let res = self.new_entry_from_text(&format!("foo = {}", buff)).value(self);
        self.detach(res);
        res
    }

    pub fn new_value_array(&mut self, entries: impl Iterator<Item=ast::Value>) -> ast::Value {
        let buff = join(self, entries, '[', ']');
        let res = self.new_entry_from_text(&format!("foo = {}", buff)).value(self);
        self.detach(res);
        res
    }

    pub fn new_entry(&mut self, keys: impl Iterator<Item=ast::Key>, value: ast::Value) -> ast::Entry {
        let mut buff = String::new();
        join_to(self, &mut buff, keys, ".", "", "");
        buff.push_str(" = ");
        value.cst().write_text(self, &mut buff);
        self.new_entry_from_text(&buff)
    }

    pub fn new_entry_from_text(&mut self, text: &str) -> ast::Entry {
        let entry = self.new_doc(text).entries(self).next().unwrap();
        self.detach(entry);
        entry
    }

    pub fn new_doc(&mut self, text: &str) -> ast::Doc {
        self.assert_edit();
        let new_root = self.tree.tree.new_internal(DOC);
        parser::parse(text, &mut self.tree, new_root);
        ast::Doc::cast(CstNode(new_root), self)
            .unwrap()
    }

    pub fn new_table(
        &mut self,
        keys: impl Iterator<Item=ast::Key>,
        entries: impl Iterator<Item=ast::Entry>,
    ) -> ast::Table {
        self.table_impl(keys, entries, "[", "]")
    }

    pub fn new_array_table(
        &mut self,
        keys: impl Iterator<Item=ast::Key>,
        entries: impl Iterator<Item=ast::Entry>,
    ) -> ast::Table {
        self.table_impl(keys, entries, "[[", "]]")
    }

    fn table_impl(
        &mut self,
        keys: impl Iterator<Item=ast::Key>,
        entries: impl Iterator<Item=ast::Entry>,
        left: &str, right: &str,
    ) -> ast::Table {
        let mut buff = String::new();
        buff.push_str(left);
        join_to(self, &mut buff, keys, ".", "", "");
        buff.push_str(right);
        for entry in entries {
            buff.push('\n');
            entry.cst().write_text(self, &mut buff);
        }
        let res = self.new_doc(&buff).tables(self).next().unwrap();
        self.detach(res);
        res
    }

    fn new_ws(&mut self, ws: &str) -> CstNode {
        let idx = self.tree.intern.intern(ws);
        CstNode(self.tree.tree.new_leaf((WHITESPACE, idx)))
    }
}

pub fn join<A: Into<CstNode>>(
    doc: &TomlDoc,
    items: impl Iterator<Item=A>,
    left: char, right: char,
) -> String {
    let mut buff = String::new();
    buff.push(left);
    join_to(doc, &mut buff, items, ", ", " ", " ");
    buff.push(right);

    buff
}

pub fn join_to<A: Into<CstNode>>(
    doc: &TomlDoc,
    buff: &mut String,
    items: impl Iterator<Item=A>,
    sep: &str,
    before_first: &str, after_last: &str,
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
        item.into().write_text(doc, buff);
    }
    if !first {
        buff.push_str(after_last);
    }
}

fn escaped_key(key: &str) -> String {
    if key.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            key.to_string()
        } else {
        format!("{:?}", key)
    }
}
