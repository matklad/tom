use std::{
    collections::HashMap,
    borrow::Cow,
};
use tom::{ast, TomlDoc};

use Result;

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
    fn from_entries(
        doc: &TomlDoc,
        name: String,
        entries: impl Iterator<Item=ast::Entry>,
        prefix: usize,
    ) -> Option<Dependency> {
        let mut url = None;
        let mut branch = None;
        let mut version = None;
        let mut optional = false;
        for e in entries {
            match last_key(doc, e, prefix)?.as_ref() {
                "git" => url = Some(e.value(doc).as_string(doc)?.into_owned()),
                "version" => version = Some(e.value(doc).as_string(doc)?.into_owned()),
                "branch" => branch = Some(e.value(doc).as_string(doc)?.into_owned()),
                "optional" => optional = e.value(doc).as_bool(doc)?,
                _ => return None,
            }
        }
        let source = match url {
            Some(url) => DependencySource::Git { url, version, branch },
            None => match (version, branch) {
                (Some(version), None) => DependencySource::Version(version),
                _ => return None,
            }
        };
        Some(Dependency { name, optional, source })
    }

    fn from_entry(doc: &TomlDoc, entry: ast::Entry) -> Option<Dependency> {
        let name = entry.keys(doc).last().unwrap().name(doc).into_owned();
        let value = entry.value(doc);
        match value.kind(doc) {
            ast::ValueKind::StringLit(s) => {
                let source = DependencySource::Version(s.value(doc).into_owned());
                Some(Dependency { name, optional: false, source })
            }
            ast::ValueKind::Dict(d) => Dependency::from_entries(doc, name, d.entries(doc), 0),
            _ => None,
        }
    }
}

fn last_key(doc: &TomlDoc, node: impl ast::KeyOwner, prefix: usize) -> Option<Cow<str>> {
    let mut keys = node.keys(doc).skip(prefix);
    let first = keys.next()?;
    if keys.next().is_some() {
        return None;
    }
    Some(first.name(doc))
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
            );
            for (name, group) in group(&self.doc, deps_group.members.iter().map(|&e| e), 1) {
                let name = name.into_owned();
                res.extend(
                    Dependency::from_entries(&self.doc, name, group.members.into_iter(), group.prefix)
                )
            }
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
            let header = table.header(&self.doc);
            if compare_keys(&self.doc, header, &["dependencies"]) {
                res.extend(table.entries(&self.doc)
                    .filter(|e| e.keys(&self.doc).count() == 1)
                    .filter_map(|entry| {
                        Dependency::from_entry(&self.doc, entry)
                    }));
                for (name, group) in group(&self.doc, table.entries(&self.doc), 0) {
                    let name = name.into_owned();
                    res.extend(
                        Dependency::from_entries(&self.doc, name, group.members.into_iter(), group.prefix)
                    )
                }
            }

            if header.keys(&self.doc).count() == 2 && header.keys(&self.doc).next().unwrap().name(&self.doc) == "dependencies" {
                let dep_name = header.keys(&self.doc).nth(1).unwrap().name(&self.doc).into_owned();
                res.extend(Dependency::from_entries(&self.doc, dep_name, table.entries(&self.doc), 0));
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
