use tom::{
    TomlFile, Factory, Edit,
    ast::{self, AstNode},
};

pub struct CargoToml<'f> {
    toml: ast::File<'f>,
    factory: &'f Factory,
    edit: Edit<'f>,
    dependencies: Option<ast::Table<'f>>,
}

impl<'f> CargoToml<'f> {
    pub fn new(toml: &'f TomlFile, factory: &'f Factory) -> CargoToml<'f> {
        CargoToml {
            toml: toml.ast(),
            factory,
            edit: Edit::new(toml),
            dependencies: None,
        }
    }

    pub fn finish(self) -> String {
        self.edit.finish()
    }

    pub fn add_dependency(&mut self, name: &str, version: &str) {
        let table = self.dependencies_table();
        let dep = self.factory.key_val(
            name,
            self.factory.val_string(version),
        );
        self.edit.append_child(table, dep);
    }

    fn dependencies_table(&mut self) -> ast::Table<'f> {
        if self.dependencies.is_none() {
            let deps = self.toml
                .find_table_by_key("dependencies")
                .unwrap_or_else(|| self.insert_empty_dependencies_table());
            self.dependencies = Some(deps)
        }
        self.dependencies.unwrap()
    }

    fn insert_empty_dependencies_table(&mut self) -> ast::Table<'f> {
        let new_table =
            self.factory
                .table()
                .with_name("dependencies")
                .build();

        match self.package_table() {
            None => self.edit.append_child(self.toml, new_table),
            Some(pkg) => self.edit.insert_sibling(pkg, new_table),
        }

        new_table
    }

    fn package_table(&self) -> Option<ast::Table<'f>> {
        self.toml.find_table_by_key("package")
            .or_else(|| self.toml.find_table_by_key("project"))
    }
}
