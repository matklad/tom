use ast::{AstNode, AstChildren};
use ast::{KeyVal, Dict, Table, ArrayTable, TableHeader, File};

pub trait KeyValueOwner<'p>: AstNode<'p> {
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

pub trait TableHeaderOwner<'p>: AstNode<'p> {
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

impl<'p> File<'p> {
    pub fn find_table(&self, name: &str) -> Option<Table<'p>> {
        self.tables()
            .find(|t| {
                let mut keys = t.header().keys();
                match keys.next() {
                    Some(key) => key.node().text() == name && keys.next().is_none(),
                    None => false,
                }
            })
    }
}
