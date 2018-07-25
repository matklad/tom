use std::{
    collections::HashMap,
    borrow::Cow,
};
use tom::{ast, IntoValue, Position::*, TomlDoc};

use {Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub optional: bool,
    pub source: DependencySource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DependencySource {
    Version(String),
    Git {
        url: String,
        version: Option<String>,
        branch: Option<String>,
    },
}

pub struct CargoToml {
    doc: TomlDoc,
}

impl Dependency {
    fn from_entry(doc: &TomlDoc, entry: ast::Entry) -> Option<Dependency> {
        let name = entry.keys(doc).last().unwrap().name(doc).into_owned();
        let value = entry.value(doc);
        let mut optional = false;
        let source = match value.kind(doc) {
            ast::ValueKind::StringLit(s) => {
                DependencySource::Version(s.value(doc).into_owned())
            }
            ast::ValueKind::Dict(d) => {
                let mut url = None;
                let mut branch = None;
                let mut version = None;
                for e in d.entries(doc) {
                    match single_key(doc, e)?.as_ref() {
                        "git" => url = Some(string_value(doc, e)?),
                        "version" => version = Some(string_value(doc, e)?),
                        "branch" => branch = Some(string_value(doc, e)?),
                        "optional" => optional = bool_value(doc, e)?,
                        _ => return None,
                    }
                }
                DependencySource::Git { url: url?, version, branch }
            }
            _ => return None,
        };
        Some(Dependency { name, optional, source })
    }
}

fn single_key(doc: &TomlDoc, node: impl ast::KeyOwner) -> Option<Cow<str>> {
    let mut keys = node.keys(doc);
    let first = keys.next()?;
    if keys.next().is_some() {
        return None;
    }
    Some(first.name(doc))
}

fn string_value(doc: &TomlDoc, node: ast::Entry) -> Option<String> {
    match node.value(doc).kind(doc) {
        ast::ValueKind::StringLit(l) => Some(l.value(doc).into_owned()),
        _ => None
    }
}

fn bool_value(doc: &TomlDoc, node: ast::Entry) -> Option<bool> {
    match node.value(doc).kind(doc) {
        ast::ValueKind::Bool(l) => Some(l.value(doc)),
        _ => None
    }
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
