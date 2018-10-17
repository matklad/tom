mod generated;

use std::borrow::Cow;

use ::{ast, AstNode, AstChildren};
// use {ast, , AstNode, Position::*, TomlDoc};
pub use self::generated::*;

pub trait EntryOwner<'a>: AstNode<'a> {
    fn entries(self) -> AstChildren<'a, ast::Entry<'a>>;
    // fn append_entry(self, entry: ast::Entry);
}

pub trait TableHeaderOwner<'a>: AstNode<'a> {
    fn header(self) -> ast::TableHeader<'a>;
}

pub trait KeyOwner<'a>: AstNode<'a> {
    fn keys(self) -> AstChildren<'a, ast::Key<'a>>;
}

// impl<'a> ast::Key<'a> {
//     pub fn name(self) -> Cow<str> {
//         match self.kind(doc) {
//             ast::KeyKind::StringLit(lit) => lit.value(doc),
//             ast::KeyKind::BareKey(key) => Cow::from(key.text(doc)),
//         }
//     }
// }

impl<'a> ast::StringLit<'a> {
    pub fn value(self) -> Cow<'a, str> {
        //TODO: broken completely
        let text = self.text();
        let len = text.len();
        Cow::from(&text[1..len - 1])
    }
}

impl<'a> ast::Bool<'a> {
    pub fn value(self) -> bool {
        self.text() == "true"
    }
}

impl<'a> ast::Number<'a> {
    pub fn value(self) -> i64 {
        self.text().parse().unwrap()
    }
}

impl<'a> ast::DateTime<'a> {
    // chrono?
    pub fn value(self) -> ::std::time::SystemTime {
        unimplemented!()
    }
}

impl<'a> ast::Value<'a> {
    pub fn as_string(self) -> Option<Cow<'a, str>> {
        match self.kind() {
            ast::ValueKind::StringLit(l) => Some(l.value()),
            _ => None,
        }
    }

    pub fn as_bool(self) -> Option<bool> {
        match self.kind() {
            ast::ValueKind::Bool(l) => Some(l.value()),
            _ => None,
        }
    }

    pub fn as_i64(self) -> Option<i64> {
        match self.kind() {
            ast::ValueKind::Number(l) => Some(l.value()),
            _ => None,
        }
    }
}

impl<'a> EntryOwner<'a> for ast::Dict<'a> {
    fn entries(self) -> AstChildren<'a, ast::Entry<'a>> {
        self.entries()
    }

//     fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
//         match self.entries(doc).last() {
//             Some(old_entry) => {
//                 let comma = doc.new_comma();
//                 doc.insert(comma, After(old_entry.into()));
//                 doc.insert(entry, After(comma));
//             }
//             None => {
//                 let l_curly = self.cst().children(doc).first().unwrap();
//                 doc.insert(entry, After(l_curly));
//                 return;
//             }
//         }
//     }
}

impl<'a> EntryOwner<'a> for ast::Table<'a> {
    fn entries(self) -> AstChildren<'a, ast::Entry<'a>> {
        self.entries()
    }
    // fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
    //     doc.insert(entry, AppendTo(self.cst()));
    // }
}

impl<'a> EntryOwner<'a> for ast::ArrayTable<'a> {
    fn entries(self) -> AstChildren<'a, ast::Entry<'a>> {
        self.entries()
    }
    // fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
    //     doc.insert(entry, AppendTo(self.cst()));
    // }
}

impl<'a> EntryOwner<'a> for ast::Doc<'a> {
    fn entries(self) -> AstChildren<'a, ast::Entry<'a>> {
        self.entries()
    }
    // fn append_entry(self, doc: &mut TomlDoc, entry: Entry) {
    //     match self.entries(doc).last() {
    //         Some(old_entry) => doc.insert(entry, After(old_entry.into())),
    //         None => doc.insert(entry, PrependTo(self.into())),
    //     }
    // }
}

impl<'a> TableHeaderOwner<'a> for ast::Table<'a> {
    fn header(self) -> ast::TableHeader<'a> {
        self.header()
    }
}

impl<'a> TableHeaderOwner<'a> for ast::ArrayTable<'a> {
    fn header(self) -> ast::TableHeader<'a> {
        self.header()
    }
}

impl<'a> KeyOwner<'a> for ast::TableHeader<'a> {
    fn keys(self) -> AstChildren<'a, ast::Key<'a>> {
        self.keys()
    }
}

impl<'a> KeyOwner<'a> for ast::Entry<'a> {
    fn keys(self) -> AstChildren<'a, ast::Key<'a>> {
        self.keys()
    }
}
