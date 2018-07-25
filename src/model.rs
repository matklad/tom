use std::collections::BTreeMap;
use {TomlDoc, CstNode, ast, visitor::{visitor, process_children}};

pub enum Item {
    Map(Map),
    Array(Vec<Item>),
    Integer(i64),
    Float(f64),
    Bool(bool),
    DateTime,
    String(String),
}

pub struct Map {
    map: BTreeMap<String, (ast::Key, Item)>,
}

impl Map {
    fn new() -> Map {
        Map { map: BTreeMap::new() }
    }

    pub fn entries(&self) -> impl Iterator<Item=(&str, &Item)> {
        self.map.iter()
            .map(|(k, (_, v))| (k.as_str(), v))
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
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
                for item in arr.iter() {
                    if !first {
                        buff.push_str(",");
                    }
                    first = false;
                    item.write_string(buff);
                }
                buff.push_str("]");
            }
            Item::Integer(value) => buff.push_str(&value.to_string()),
            Item::Float(value) => buff.push_str(&value.to_string()),
            Item::Bool(value) => buff.push_str(&value.to_string()),
            Item::DateTime => buff.push_str("TODO:date-time"),
            Item::String(value) => buff.push_str(&format!("{:?}", value.to_string())),
        }
    }
}

pub(crate) fn from_doc(doc: &TomlDoc) -> Item {
    let mut root = Item::Map(Map::new());
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
        })
        .visit::<ast::ArrayTable, _>(|item, table| {
            match insert_into(doc, *item, table.header(doc).keys(doc)) {
                Some(tbl) => {
                    let mut new_item = Item::Map(Map::new());
                    fill(doc, table.cst(), &mut new_item);

                    match tbl {
                        Item::Map(map) if map.is_empty() => (),
                        Item::Array(array) => {
                            array.push(new_item);
                            return;
                        }
                        _ => return,
                    }
                    *tbl = Item::Array(vec![new_item])
                }
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
                curr = &mut map.map.entry(key_name).or_insert_with(|| {
                    (key, Item::Map(Map::new()))
                }).1
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
        ast::ValueKind::Array(a) => Item::Array(
            a.values(doc).map(|val| from_value(doc, val)).collect(),
        ),
        ast::ValueKind::Dict(_) => Item::Map(Map {
            map: BTreeMap::new(), // TODO
        }),
        ast::ValueKind::Number(n) => Item::Integer(n.value(doc)),
        ast::ValueKind::Bool(b) => Item::Bool(b.value(doc)),
        ast::ValueKind::DateTime(d) => Item::DateTime,
        ast::ValueKind::StringLit(s) => Item::String(s.value(doc).to_string()),
    }
}
