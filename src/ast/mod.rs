mod generated;

use std::borrow::Cow;

pub use self::generated::*;
use {ast, AstChildren, AstNode, CstNode, Position::*, TomlDoc};

pub trait EntryOwner: AstNode {
    fn entries(self, doc: &TomlDoc) -> AstChildren<ast::Entry>;
    fn append_entry(self, doc: &mut TomlDoc, entry: ast::Entry);
}

pub trait TableHeaderOwner: AstNode {
    fn header(self, doc: &TomlDoc) -> ast::TableHeader;
}

pub trait KeyOwner: AstNode {
    fn keys(self, doc: &TomlDoc) -> AstChildren<ast::Key>;
}

impl ast::Key {
    pub fn name(self, doc: &TomlDoc) -> Cow<str> {
        match self.kind(doc) {
            ast::KeyKind::StringLit(lit) => lit.value(doc),
            ast::KeyKind::BareKey(key) => Cow::from(key.text(doc)),
        }
    }
}

impl ast::StringLit {
    pub fn value(self, doc: &TomlDoc) -> Cow<str> {
        //TODO: broken completely
        let text = self.text(doc);
        let len = text.len();
        Cow::from(&text[1..len - 1])
    }
}

impl ast::Bool {
    pub fn value(self, doc: &TomlDoc) -> bool {
        self.text(doc) == "true"
    }
}

impl ast::Number {
    pub fn value(self, _doc: &TomlDoc) -> i64 {
        unimplemented!()
    }
}

impl ast::DateTime {
    // chrono?
    pub fn value(self, _doc: &TomlDoc) -> ::std::time::SystemTime {
        unimplemented!()
    }
}

impl EntryOwner for ast::Dict {
    fn entries(self, doc: &TomlDoc) -> AstChildren<ast::Entry> {
        self.entries(doc)
    }

    fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
        match self.entries(doc).last() {
            Some(old_entry) => {
                let comma = doc.new_comma();
                doc.insert(comma, After(old_entry.into()));
                doc.insert(entry, After(comma));
            }
            None => {
                let l_curly = self.cst().children(doc).first().unwrap();
                doc.insert(entry, After(l_curly));
                return;
            }
        }
    }
}

impl EntryOwner for ast::Table {
    fn entries(self, doc: &TomlDoc) -> AstChildren<ast::Entry> {
        self.entries(doc)
    }
    fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
        append_table_entry(doc, self.cst(), entry)
    }
}

impl EntryOwner for ast::ArrayTable {
    fn entries(self, doc: &TomlDoc) -> AstChildren<ast::Entry> {
        self.entries(doc)
    }
    fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
        append_table_entry(doc, self.cst(), entry)
    }
}

fn append_table_entry(doc: &mut TomlDoc, table: CstNode, entry: Entry) {
    doc.insert(entry, AppendTo(table));
}

impl EntryOwner for ast::Doc {
    fn entries(self, doc: &TomlDoc) -> AstChildren<ast::Entry> {
        self.entries(doc)
    }

    fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
        match self.entries(doc).last() {
            Some(old_entry) => doc.insert(entry, After(old_entry.into())),
            None => doc.insert(entry, PrependTo(self.into())),
        }
    }
}

impl TableHeaderOwner for ast::Table {
    fn header(self, doc: &TomlDoc) -> ast::TableHeader {
        self.header(doc)
    }
}

impl TableHeaderOwner for ast::ArrayTable {
    fn header(self, doc: &TomlDoc) -> ast::TableHeader {
        self.header(doc)
    }
}

impl KeyOwner for ast::TableHeader {
    fn keys(self, doc: &TomlDoc) -> AstChildren<ast::Key> {
        self.keys(doc)
    }
}

impl KeyOwner for ast::Entry {
    fn keys(self, doc: &TomlDoc) -> AstChildren<ast::Key> {
        self.keys(doc)
    }
}
