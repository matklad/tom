use std::collections::BTreeMap;
use {TomlDoc, CstNode, ast, visitor::{visitor, process_children}};

pub enum Item {
    Map(Map),
    Array(Array),
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

pub struct Map {
    #[allow(unused)]
    flavor: MapFlavor,
    entries: BTreeMap<String, Item>,
}

impl Map {
    pub fn entries(&self) -> impl Iterator<Item=(&String, &Item)> {
        self.entries.iter()
    }
}

pub struct Array {
    #[allow(unused)]
    flavor: ArrayFlavor,
    items: Vec<Item>,
}

impl Array {
    pub fn items(&self) -> impl Iterator<Item=&Item> {
        self.items.iter()
    }
}

impl Item {
    pub fn to_string(&self) -> String {
        let mut buff = String::new();
        self.write_string(&mut buff);
        buff
    }

    fn write_string(&self, buff: &mut String) {
        match self {
            Item::Map(map) => {
                buff.push_str("{");
                let mut first = true;
                for (k, v) in map.entries() {
                    if !first {
                        buff.push_str(",");
                    }
                    first = false;
                    buff.push_str(&format!("{:?}:", k));
                    v.write_string(buff);
                }
                buff.push_str("}");
            }
            Item::Array(arr) => {
                buff.push_str("[");
                let mut first = true;
                for item in arr.items() {
                    if !first {
                        buff.push_str(",");
                    }
                    first = false;
                    item.write_string(buff);
                }
                buff.push_str("]");
            }
            Item::Integer { value, .. } => buff.push_str(&value.to_string()),
            Item::Float { value, .. } => buff.push_str(&value.to_string()),
            Item::Bool { value, .. } => buff.push_str(&value.to_string()),
            Item::DateTime { .. } => buff.push_str("TODO:date-time"),
            Item::String { value, .. } => buff.push_str(&format!("{:?}", value.to_string())),
        }
    }
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
    let mut root = Item::Map(Map {
        flavor: MapFlavor::Root(doc.ast()),
        entries: BTreeMap::new(),
    });
    fill(doc, doc.cst(), &mut root);
    root
}

fn fill(doc: &TomlDoc, node: CstNode, item: &mut Item) {
    let v = visitor(item)
        .visit::<ast::Entry, _>(|item, entry| {
            match insert_into(doc, *item, entry.keys(doc)) {
                Some(tbl) => *tbl = from_value(doc, entry.value(doc)),
                None => return,
            }
        })
        .visit::<ast::Table, _>(|item, table| {
            match insert_into(doc, *item, table.header(doc).keys(doc)) {
                Some(tbl) => fill(doc, table.cst(), tbl),
                None => return,
            }
        });
    process_children(node, doc, v);
}

fn insert_into<'a>(
    doc: &TomlDoc,
    item: &'a mut Item,
    keys: impl Iterator<Item=ast::Key>,
) -> Option<&'a mut Item> {
    let mut curr = item;
    for key in keys {
        let prev = curr; // nll shenanigans
        match prev {
            Item::Map(map) => {
                let key_name = key.name(doc).to_string();
                curr = map.entries.entry(key_name).or_insert_with(|| {
                    Item::Map(Map {
                        // TODO: append Keyed
                        flavor: MapFlavor::Keyed(vec![key]),
                        entries: BTreeMap::new(),
                    })
                })
            }
            _ => {
                // TODO:
                return None;
            }
        };
    }
    Some(curr)
}

fn from_value(doc: &TomlDoc, value: ast::Value) -> Item {
    match value.kind(doc) {
        ast::ValueKind::Array(a) => Item::Array(Array {
            flavor: ArrayFlavor::Inline(a),
            items: a.values(doc).map(|val| from_value(doc, val)).collect(),
        }),
        ast::ValueKind::Dict(d) => Item::Map(Map {
            flavor: MapFlavor::Inline(d),
            entries: BTreeMap::new(), // TODO
        }),
        ast::ValueKind::Number(n) => Item::Integer {
            ast: n,
            value: n.value(doc),
        },
        ast::ValueKind::Bool(b) => Item::Bool {
            ast: b,
            value: b.value(doc),
        },
        ast::ValueKind::DateTime(d) => Item::DateTime {
            ast: d,
        },
        ast::ValueKind::StringLit(s) => Item::String {
            ast: s,
            value: s.value(doc).to_string(),
        }
    }
}
