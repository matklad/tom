use tom::{
    TomlFile, Factory, Edit, Position,
    ast,
};

pub struct CargoTomlManipulator<'f> {
    toml: ast::File<'f>,
    factory: &'f Factory,
    edit: Edit<'f>,
    dependencies: Option<ast::Table<'f>>,
}

impl<'f> CargoTomlManipulator<'f> {
    pub fn new(toml: &'f TomlFile, factory: &'f Factory) -> CargoTomlManipulator<'f> {
        CargoTomlManipulator {
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
        self.edit.insert(dep, Position::end_of(table));
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

        let position = match self.package_table() {
            None => Position::end_of(self.toml),
            Some(pkg) => Position::after(pkg),
        };
        self.edit.insert(new_table, position);

        new_table
    }

    fn package_table(&self) -> Option<ast::Table<'f>> {
        self.toml.find_table_by_key("package")
            .or_else(|| self.toml.find_table_by_key("project"))
    }
}
