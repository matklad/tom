use std::iter;

use tom::{
    ast, TomlDoc, Position::*, IntoValue
};

pub struct CargoToml(TomlDoc);


enum DependencyNode {
    Vers(ast::StringLit),
    Table(ast::Table),
    Dict(ast::Dict),
}

pub struct Dependency {
    pub name: String,
    pub source: DependencySource,
    pub optional: bool,
}

pub enum DependencySource {
    Version(String),
    Git {
        url: String,
        version: Option<String>,
        branch: Option<String>,
    },
}

impl CargoToml {
    pub fn new(text: &str) -> CargoToml {
        let mut doc = TomlDoc::new(text);
        doc.start_edit();
        CargoToml(doc)
    }

    pub fn finish(self) -> String {
        self.0.cst().get_text(&self.0)
    }

    pub fn add_dependency(&mut self, name: &str, version: &str) {
        let table = self.dependencies_table();
        let dep = new_entry(&mut self.0, name, version);
        self.0.insert(dep, AppendTo(table.into()));
    }

    pub fn update_dependency(&mut self, dep: Dependency) {
        let toml_dep = self.find_or_insert_dep(&dep.name);
        self.merge(toml_dep, dep);
    }

    fn dependencies_table(&mut self) -> ast::Table {
        self.find_table("dependencies").unwrap_or_else(|| {
            let new_table = self.0.new_table_from_text("[dependencies]");
            let position = match self.package_table() {
                Some(pkg) => After(pkg.into()),
                None => AppendTo(self.0.cst()),
            };
            self.0.insert(new_table, position);
            new_table
        })
    }

    fn package_table(&self) -> Option<ast::Table> {
        self.find_table("package")
            .or_else(|| self.find_table("project"))
    }

    fn find_or_insert_dep(&mut self, name: &str) -> DependencyNode {
        if let Some(table) = find_table(&self.0, &["dependencies", &name]) {
            return DependencyNode::Table(table);
        }
        let deps_table = self.dependencies_table();
        let entry = find_entry(&self.0, deps_table, name).unwrap_or_else(|| {
            let new_dep = new_entry(&mut self.0, name, "");
            self.0.insert(new_dep, AppendTo(deps_table.into()));
            new_dep
        });
        match entry.value(&self.0).kind(&self.0) {
            ast::ValueKind::Dict(d) => DependencyNode::Dict(d),
            ast::ValueKind::StringLit(l) => DependencyNode::Vers(l),
            _ => panic!("invalid dependency")
        }
    }

    fn merge(&mut self, dep_node: DependencyNode, dep: Dependency) {
        match dep_node {
            DependencyNode::Vers(v) => {
                match &dep.source {
                    DependencySource::Version(vers) if !dep.optional => {
                        let new_vers = self.0.new_value(vers.as_str());
                        self.0.replace(v, new_vers);
                        return;
                    }
                    _ => ()
                };
                let old_value = v.cst().parent(&self.0).unwrap();
                let empty = self.0.new_value_from_text("{}");
                self.0.replace(old_value, empty);
                match empty.kind(&self.0) {
                    ast::ValueKind::Dict(d) => self.merge_into(d, dep),
                    _ => unreachable!(),
                }
            }
            DependencyNode::Table(table) => self.merge_into(table, dep),
            DependencyNode::Dict(dict) => self.merge_into(dict, dep),
        }
    }

    fn merge_into(
        &mut self,
        table: impl ast::EntryOwner,
        dep: Dependency,
    ) {

        match &dep.source {
            DependencySource::Version(version) => {
                update_entry(&mut self.0, table, "version", version.as_str());
            }
            DependencySource::Git { url, version, branch } => {
                update_entry(&mut self.0, table, "git", url.as_str());

                if let Some(branch) = branch {
                    update_entry(&mut self.0, table, "branch", branch.as_str());
                }
                if let Some(version) = version {
                    update_entry(&mut self.0, table, "version", version.as_str());
                }
            }
        }
        if dep.optional {
            update_entry(&mut self.0, table, "optional", true);
        }
    }

    // NB: does not handle `dependencies = { foo = "1.0.0" }` case
    fn find_table(&self, name: &str) -> Option<ast::Table> {
        find_table(&self.0, &[name])
    }
}

fn find_entry(doc: &TomlDoc, table: impl ast::EntryOwner, key: &str) -> Option<ast::Entry> {
    table.entries(doc).find(|&entry| compare_keys(doc, entry, &[key]))
}

fn update_entry(doc: &mut TomlDoc, table: impl ast::EntryOwner, key: &str, new_value: impl IntoValue) {
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

fn new_entry(doc: &mut TomlDoc, key: &str, value: impl IntoValue) -> ast::Entry {
    let key = doc.new_key(key);
    let value = doc.new_value(value);
    doc.new_entry(iter::once(key), value)
}

fn find_table(doc: &TomlDoc, keys: &[&str]) -> Option<ast::Table> {
    doc.ast().tables(doc).find(|t| compare_keys(doc, t.header(doc), keys))
}

fn compare_keys(doc: &TomlDoc, el: impl ast::KeyOwner, keys: &[&str]) -> bool {
    let keys = keys.iter().cloned();
    let el_keys = el.keys(doc).map(|key| key.name(doc));
    el_keys.eq(keys)
}
