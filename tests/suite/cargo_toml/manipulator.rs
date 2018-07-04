use std::iter;
use tom::{
    TomlDoc,
    ast,
};


enum DependencyNode<'f> {
    Vers(ast::StringLit<'f>),
    Table(ast::Table<'f>),
    Dict(ast::Dict<'f>),
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

pub struct CargoTomlManipulator<'f> {
    doc: ast::Doc<'f>,
    factory: &'f Factory,
    edit: Edit<'f>,
    dependencies: Option<ast::Table<'f>>,
}

impl<'f> CargoTomlManipulator<'f> {
    pub fn new(toml: &'f TomlDoc, factory: &'f Factory) -> CargoTomlManipulator<'f> {
        CargoTomlManipulator {
            doc: toml.ast(),
            factory,
            edit: Edit::new(toml),
            dependencies: None,
        }
    }

    pub fn finish(self) -> String {
        self.edit.finish()
    }

    pub fn update_dependency(&mut self, dep: Dependency) {
        let toml_dep = self.find_or_insert_dep(&dep.name);
        self.merge(toml_dep, dep);
    }

    fn find_or_insert_dep(&mut self, name: &str) -> DependencyNode<'f> {
        if let Some(table) = find_table(self.doc, &["dependencies", &name]) {
            return DependencyNode::Table(table);
        }
        let deps_table = self.dependencies_table();
        match find_entry(deps_table, name) {
            None => {
                let new_dep = self.entry(
                    name,
                    self.factory.value_string(""),
                );
                self.edit.insert(new_dep, Position::end_of(deps_table));
                DependencyNode::Vers(
                    match new_dep.value().kind() {
                        ast::ValueKind::StringLit(d) => d,
                        _ => unreachable!(),
                    }
                )
            }
            Some(entry) => match entry.value().kind() {
                ast::ValueKind::Dict(d) => DependencyNode::Dict(d),
                ast::ValueKind::StringLit(l) => DependencyNode::Vers(l),
                _ => panic!("invalid dependency")
            }
        }
    }

    fn merge(&mut self, dep_node: DependencyNode<'f>, dep: Dependency) {
        let mut entries = Vec::new();
        match &dep.source {
            DependencySource::Version(vers) => {
                entries.push(
                    self.entry("version", self.factory.value_string(&vers))
                )
            }
            DependencySource::Git { url, version, branch } => {
                entries.push(
                    self.entry("git", self.factory.value_string(url))
                );
                if let Some(branch) = branch {
                    entries.push(
                        self.entry("branch", self.factory.value_string(branch))
                    );
                }
                if let Some(version) = version {
                    entries.push(
                        self.entry("version", self.factory.value_string(version))
                    );
                }
            }
        }
        if dep.optional {
            entries.push(
                self.entry("optional", self.factory.value_bool(true))
            );
        }

        match dep_node {
            DependencyNode::Vers(v) => {
                let replacement = match &dep.source {
                    DependencySource::Version(vers) if !dep.optional =>
                        self.factory.value_string(&vers),
                    _ => self.factory.value_dict(entries.into_iter()),
                };

                self.edit.replace(v, replacement);
            }
            DependencyNode::Table(table) => self.merge_into(table, entries, ""),
            DependencyNode::Dict(dict) => self.merge_into(dict, entries, ","),
        }
    }

    fn entry(&self, name: &str, val: ast::Value) -> ast::Entry<'f> {
        let key = self.factory.key(name);
        self.factory.entry(iter::once(key), val)
    }

    fn merge_into(
        &mut self,
        table: impl ast::EntryOwner<'f>,
        entries: Vec<ast::Entry<'f>>,
        sep: &str,
    ) {
        fn eq_entries(a: ast::Entry, b: ast::Entry) -> bool {
            a.keys().map(|k| k.name()).eq(b.keys().map(|k| k.name()))
        }

        for old in table.entries() {
            if let Some(new) = entries.iter().find(|&&new| eq_entries(old, new)) {
                self.edit.replace(old.value(), new.value());
            }
        }
        let pos = table.entries().last().map(|e| e.cst()).unwrap_or_else(|| {
            table.cst().children().next().unwrap()
        });
        for new in entries {
            if table.entries().find(|&old| eq_entries(old, new)).is_none() {
                self.edit.insert(new, Position::After(pos));
            }
        }
    }

    pub fn add_dependency(&mut self, name: &str, version: &str) {
        let table = self.dependencies_table();
        let key = self.factory.key(name);
        let dep = self.factory.entry(
            iter::once(key),
            self.factory.value_string(version),
        );
        self.edit.insert(dep, Position::end_of(table));
    }

    fn dependencies_table(&mut self) -> ast::Table<'f> {
        if self.dependencies.is_none() {
            let deps = self
                .find_table("dependencies")
                .unwrap_or_else(|| self.insert_empty_dependencies_table());
            self.dependencies = Some(deps)
        }
        self.dependencies.unwrap()
    }

    fn insert_empty_dependencies_table(&mut self) -> ast::Table<'f> {
        let new_table = self.factory.table(
            iter::once(self.factory.key("dependencies")),
            iter::empty(),
        );

        let position = match self.package_table() {
            None => Position::end_of(self.doc),
            Some(pkg) => Position::after(pkg),
        };
        self.edit.insert(new_table, position);

        new_table
    }

    fn package_table(&self) -> Option<ast::Table<'f>> {
        self.find_table("package")
            .or_else(|| self.find_table("project"))
    }

    // NB: does not handle `dependencies = { foo = "1.0.0" }` case
    fn find_table(&self, name: &str) -> Option<ast::Table<'f>> {
        find_table(self.doc, &[name])
    }
}

fn find_entry<'d>(table: impl ast::EntryOwner<'d>, key: &str) -> Option<ast::Entry<'d>> {
    table.entries().find(|&entry| compare_keys(entry, &[key]))
}

fn find_table<'d>(doc: ast::Doc<'d>, keys: &[&str]) -> Option<ast::Table<'d>> {
    doc.tables().find(|t| compare_keys(t.header(), keys))
}

fn compare_keys<'d>(el: impl ast::KeyOwner<'d>, keys: &[&str]) -> bool {
    let keys = keys.iter().cloned();
    let el_keys = el.keys().map(|key| key.name());
    el_keys.eq(keys)
}

//fn merge_dependencies(old_dep: &mut toml_edit::Item, new: &Dependency) {
//    assert!(!old_dep.is_none());
//
//    let new_toml = new.to_toml().1;
//
//    if str_or_1_len_table(old_dep) {
//        // The old dependency is just a version/git/path. We are safe to overwrite.
//        *old_dep = new_toml;
//    } else if old_dep.is_table_like() {
//        for key in &["version", "path", "git"] {
//            // remove this key/value pairs
//            old_dep[key] = toml_edit::Item::None;
//        }
//        if let Some(name) = new_toml.as_str() {
//            old_dep["version"] = toml_edit::value(name);
//        } else {
//            merge_inline_table(old_dep, &new_toml);
//        }
//    } else {
//        unreachable!("Invalid old dependency type");
//    }
//
//    if let Some(t) = old_dep.as_inline_table_mut() {
//        t.fmt()
//    }
//}
//fn merge_inline_table(old_dep: &mut toml_edit::Item, new: &toml_edit::Item) {
//    for (k, v) in new.as_inline_table()
//        .expect("expected an inline table")
//        .iter()
//        {
//            old_dep[k] = toml_edit::value(v.clone());
//        }
//}
