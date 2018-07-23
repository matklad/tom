use std::iter;

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
        for entry in self.doc.ast().entries(&self.doc) {
            if compare_keys(&self.doc, entry, &["dependencies"]) {
                match entry.value(&self.doc).kind(&self.doc) {
                    ast::ValueKind::Dict(d) => {
                        res.extend(d.entries(&self.doc).filter_map(|entry| {
                            Dependency::from_entry(&self.doc, entry)
                        }))
                    },
                    _ => {},
                }
            }
        }

        for table in self.doc.ast().tables(&self.doc) {
            if compare_keys(&self.doc, table.header(&self.doc), &["dependencies"]) {
                res.extend(table.entries(&self.doc).filter_map(|entry| {
                    Dependency::from_entry(&self.doc, entry)
                }))
            }
        }
        res
    }

    pub fn update_dependency(&mut self, dep: &Dependency) -> Result<()> {
        self.doc.start_edit();
        let toml_dep = self.find_or_insert_dep(&dep.name)?;
        self.merge(toml_dep, dep);
        self.doc.finish_edit_no_reparse();
        Ok(())
    }

    fn find_or_insert_dep(&mut self, name: &str) -> Result<DependencyNode> {
        if let Some(table) = find_table(&self.doc, &["dependencies", &name]) {
            return Ok(DependencyNode::Table(table));
        }
        let deps_table = self.find_or_insert_dependencies_table();
        let entry = find_entry(&self.doc, deps_table, name).unwrap_or_else(|| {
            let new_dep = new_entry(&mut self.doc, name, "");
            self.doc.insert(new_dep, AppendTo(deps_table.into()));
            new_dep
        });
        match entry.value(&self.doc).kind(&self.doc) {
            ast::ValueKind::Dict(d) => Ok(DependencyNode::Dict(d)),
            ast::ValueKind::StringLit(l) => Ok(DependencyNode::Vers(l)),
            _ => bail!("invalid dependency"),
        }
    }

    fn find_or_insert_dependencies_table(&mut self) -> ast::Table {
        self.find_table("dependencies").unwrap_or_else(|| {
            let new_table = self.doc.new_table_from_text("[dependencies]");
            let position = match self.package_table() {
                Some(pkg) => After(pkg.into()),
                None => AppendTo(self.doc.cst()),
            };
            self.doc.insert(new_table, position);
            new_table
        })
    }

    fn package_table(&self) -> Option<ast::Table> {
        self.find_table("package")
            .or_else(|| self.find_table("project"))
    }

    // NB: does not handle `dependencies = { foo = "1.0.0" }` case
    fn find_table(&self, name: &str) -> Option<ast::Table> {
        find_table(&self.doc, &[name])
    }

    fn merge(&mut self, dep_node: DependencyNode, dep: &Dependency) {
        match dep_node {
            DependencyNode::Vers(v) => {
                match &dep.source {
                    DependencySource::Version(vers) if !dep.optional => {
                        let new_vers = self.doc.new_value(vers.as_str());
                        self.doc.replace(v, new_vers);
                        return;
                    }
                    _ => (),
                };
                let old_value = v.cst().parent(&self.doc).unwrap();
                let empty = self.doc.new_value_from_text("{}");
                self.doc.replace(old_value, empty);
                match empty.kind(&self.doc) {
                    ast::ValueKind::Dict(d) => self.merge_into(d, dep),
                    _ => unreachable!(),
                }
            }
            DependencyNode::Table(table) => self.merge_into(table, dep),
            DependencyNode::Dict(dict) => self.merge_into(dict, dep),
        }
    }

    fn merge_into(&mut self, table: impl ast::EntryOwner, dep: &Dependency) {
        match &dep.source {
            DependencySource::Version(version) => {
                update_entry(&mut self.doc, table, "version", version.as_str());
            }
            DependencySource::Git {
                url,
                version,
                branch,
            } => {
                update_entry(&mut self.doc, table, "git", url.as_str());

                if let Some(branch) = branch {
                    update_entry(&mut self.doc, table, "branch", branch.as_str());
                }
                if let Some(version) = version {
                    update_entry(&mut self.doc, table, "version", version.as_str());
                }
            }
        }
        if dep.optional {
            update_entry(&mut self.doc, table, "optional", true);
        }
    }
}

fn find_entry(doc: &TomlDoc, table: impl ast::EntryOwner, key: &str) -> Option<ast::Entry> {
    table
        .entries(doc)
        .find(|&entry| compare_keys(doc, entry, &[key]))
}

fn find_table(doc: &TomlDoc, keys: &[&str]) -> Option<ast::Table> {
    doc.ast()
        .tables(doc)
        .find(|t| compare_keys(doc, t.header(doc), keys))
}

fn compare_keys(doc: &TomlDoc, el: impl ast::KeyOwner, keys: &[&str]) -> bool {
    let keys = keys.iter().cloned();
    let el_keys = el.keys(doc).map(|key| key.name(doc));
    el_keys.eq(keys)
}

fn new_entry(doc: &mut TomlDoc, key: &str, value: impl IntoValue) -> ast::Entry {
    let key = doc.new_key(key);
    let value = doc.new_value(value);
    doc.new_entry(iter::once(key), value)
}

fn update_entry(
    doc: &mut TomlDoc,
    table: impl ast::EntryOwner,
    key: &str,
    new_value: impl IntoValue,
) {
    match find_entry(doc, table, key) {
        Some(entry) => {
            let old_value = entry.value(doc);
            let new_value = doc.new_value(new_value);
            doc.replace(old_value, new_value)
        }
        None => {
            let new_entry = new_entry(doc, key, new_value);
            table.append_entry(doc, new_entry)
        }
    }
}
