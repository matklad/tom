use ast::{AstNode, AstChildren};
use toml_ast::{KeyVal, Dict, Table, ArrayTable, TableHeader, File};

trait KeyValueOwner<'p>: AstNode<'p> {
    fn entries(&self) -> AstChildren<'p, KeyVal<'p>> {
        AstChildren::new(self.node().children())
    }
}

impl<'p> KeyValueOwner<'p> for Dict<'p> {
}

impl<'p> KeyValueOwner<'p> for Table<'p> {
}

impl<'p> KeyValueOwner<'p> for ArrayTable<'p> {
}

impl<'p> KeyValueOwner<'p> for File<'p> {
}

trait TableHeaderOwner<'p>: AstNode<'p> {
    fn header(&self) ->  TableHeader<'p> {
        AstChildren::new(self.node().children())
            .next()
            .expect("Table without header")
    }
}

impl<'p> TableHeaderOwner<'p> for Table<'p> {
}

impl<'p> TableHeaderOwner<'p> for ArrayTable<'p> {
}
