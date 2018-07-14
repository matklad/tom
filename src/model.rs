use std::collections::HashMap;
use {TomlDoc, ast};

pub enum Item {
    Map {
        flavor: MapFlavor,
        entries: HashMap<String, Item>,
    },
    Array {
        flavor: ArrayFlavor,
        values: Vec<Item>,
    },
    Integer {
        ast: ast::Number,
        value: i64,
    },
    Float {
        ast: ast::Number,
        value: f64,
    },
    Bool {
        ast: ast::Bool,
        value: bool,
    },
    DateTime {
        ast: ast::DateTime,
        // value: ???
    },
    String {
        ast: ast::StringLit,
        value: String,
    },
}

pub enum MapFlavor {
    Root(ast::Doc),
    Table(ast::Table),
    Inline(ast::Dict),
    Keyed(Vec<ast::Key>),
}

pub enum ArrayFlavor {
    Table(Vec<ast::ArrayTable>),
    Inline(ast::Array),
}

pub(crate) fn from_doc(doc: &TomlDoc) -> Item {
    let mut entries = HashMap::new();
    Item::Map {
        flavor: MapFlavor::Root(doc.ast()),
        entries,
    }
}
