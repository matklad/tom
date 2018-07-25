use std::{
    iter,
    collections::HashMap,
    borrow::Cow,
};

use itertools::Itertools;

use {Result, Dependency, DependencySource};
use tom::{ast, IntoValue, Position::*, TomlDoc};

pub struct CargoToml {
    doc: TomlDoc,
}

enum DependencyNode {
    Vers(ast::StringLit),
    Table(ast::Table),
    Dict(ast::Dict),
}

impl CargoToml {
    pub fn new(text: &str) -> Result<CargoToml> {
        let mut doc = TomlDoc::new(text);
        if let Some(err) = doc.errors().first() {
            bail!(
                "syntax error at offset {}: {}",
                err.range().start(),
                err.message()
            );
        };

        doc.start_edit();
        Ok(CargoToml { doc })
    }

    pub fn text(&self) -> String {
        self.doc.cst().get_text(&self.doc)
    }

    pub fn dependencies(&self) -> Vec<Dependency> {
        let mut res = Vec::new();
        if let Some(deps_group) = group(&self.doc, self.doc.ast().entries(&self.doc), 0).get("dependencies") {
            res.extend(deps_group.members.iter()
                .map(|&e| e)
                .filter(|e| e.keys(&self.doc).count() == 2)
                .filter_map(|e| Dependency::from_entry(&self.doc, e))
            )
        }

        for entry in self.doc.ast().entries(&self.doc) {
            if compare_keys(&self.doc, entry, &["dependencies"]) {
                match entry.value(&self.doc).kind(&self.doc) {
                    ast::ValueKind::Dict(d) => {
                        res.extend(d.entries(&self.doc).filter_map(|entry| {
                            Dependency::from_entry(&self.doc, entry)
                        }))
                    }
                    _ => {}
                }
            }
        }

        for table in self.doc.ast().tables(&self.doc) {
            if compare_keys(&self.doc, table.header(&self.doc), &["dependencies"]) {
                res.extend(table.entries(&self.doc)
                    .filter(|e| e.keys(&self.doc).count() == 1)
                    .filter_map(|entry| {
                    Dependency::from_entry(&self.doc, entry)
                }))
            }
        }
        res
    }
}

struct EntryGroup {
    members: Vec<ast::Entry>,
    prefix: usize,
}

impl EntryGroup {
    fn empty(prefix: usize) -> EntryGroup {
        EntryGroup { members: Vec::new(), prefix }
    }

    fn name<'a>(&self, doc: &'a TomlDoc) -> Cow<'a, str> {
        self.members.first().unwrap().keys(doc).next().unwrap().name(doc)
    }
}

fn group<'a>(
    doc: &'a TomlDoc,
    entries: impl Iterator<Item=ast::Entry>,
    prefix: usize,
) -> HashMap<Cow<'a, str>, EntryGroup> {
    let mut res = HashMap::new();
    for e in entries {
        let key = e.keys(doc).skip(prefix).next();
        if let Some(key) = key {
            res.entry(key.name(doc))
                .or_insert_with(|| EntryGroup::empty(prefix + 1))
                .members.push(e)
        }
    }
    res
}

fn compare_keys(doc: &TomlDoc, el: impl ast::KeyOwner, keys: &[&str]) -> bool {
    let keys = keys.iter().cloned();
    let el_keys = el.keys(doc).map(|key| key.name(doc));
    el_keys.eq(keys)
}
