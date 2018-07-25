mod generated;

use std::borrow::Cow;

use {ast, AstChildren, AstNode, Position::*, TomlDoc};
pub use self::generated::*;

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
    pub fn value(self, doc: &TomlDoc) -> i64 {
        self.text(doc).parse().unwrap()
    }
}

impl ast::DateTime {
    // chrono?
    pub fn value(self, _doc: &TomlDoc) -> ::std::time::SystemTime {
        unimplemented!()
    }
}

impl ast::Value {
    pub fn as_string(self, doc: &TomlDoc) -> Option<Cow<str>> {
        match self.kind(doc) {
            ast::ValueKind::StringLit(l) => Some(l.value(doc)),
            _ => None,
        }
    }

    pub fn as_bool(self, doc: &TomlDoc) -> Option<bool> {
        match self.kind(doc) {
            ast::ValueKind::Bool(l) => Some(l.value(doc)),
            _ => None,
        }
    }

    pub fn as_i64(self, doc: &TomlDoc) -> Option<i64> {
        match self.kind(doc) {
            ast::ValueKind::Number(l) => Some(l.value(doc)),
            _ => None,
        }
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
        doc.insert(entry, AppendTo(self.cst()));
    }
}

impl EntryOwner for ast::ArrayTable {
    fn entries(self, doc: &TomlDoc) -> AstChildren<ast::Entry> {
        self.entries(doc)
    }
    fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
        doc.insert(entry, AppendTo(self.cst()));
    }
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
