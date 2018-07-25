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

    pub fn iter(&self) -> impl Iterator<Item=(&str, &Item)> {
        self.map.iter()
            .map(|(k, (_, v))| (k.as_str(), v))
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Item> {
        self.map.get(key).map(|(_, i)| i)
    }
}

impl Item {
    pub fn as_map(&self) -> Option<&Map> {
        match self {
            Item::Map(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Item::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Item::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Item::Integer(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Item::Float(v) => Some(*v),
            _ => None,
        }
    }

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
                for (k, v) in map.iter() {
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

pub(crate) fn from_doc(doc: &TomlDoc) -> Map {
    let mut root = Item::Map(Map::new());
    fill(doc, doc.cst(), &mut root);
    match root {
        Item::Map(map) => map,
        _ => unreachable!()
    }
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
        ast::ValueKind::Dict(d) => {
            let mut map = Item::Map(Map::new());
            fill(doc, d.cst(), &mut map);
            map
        },
        ast::ValueKind::Number(n) => Item::Integer(n.value(doc)),
        ast::ValueKind::Bool(b) => Item::Bool(b.value(doc)),
        ast::ValueKind::DateTime(_) => Item::DateTime,
        ast::ValueKind::StringLit(s) => Item::String(s.value(doc).to_string()),
    }
}
