use std::{
    borrow::Cow,
};

use ::{
    ast::{self, AstNode, AstChildren},
};

pub trait EntryOwner<'f>: AstNode<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>>;
}

pub trait KeyOwner<'f>: AstNode<'f> {
    fn keys(self) -> AstChildren<'f, ast::Key<'f>>;
}

pub trait TableHeaderOwner<'f>: AstNode<'f> {
    fn header(self) -> ast::TableHeader<'f>;
}



impl<'f> EntryOwner<'f> for ast::Dict<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for ast::Table<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for ast::ArrayTable<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for ast::Doc<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> KeyOwner<'f> for ast::TableHeader<'f> {
    fn keys(self) -> AstChildren<'f, ast::Key<'f>> { self.keys() }
}

impl<'f> KeyOwner<'f> for ast::Entry<'f> {
    fn keys(self) -> AstChildren<'f, ast::Key<'f>> { self.keys() }
}

impl<'f> TableHeaderOwner<'f> for ast::Table<'f> {
    fn header(self) -> ast::TableHeader<'f> { self.header() }
}

impl<'f> TableHeaderOwner<'f> for ast::ArrayTable<'f> {
    fn header(self) -> ast::TableHeader<'f> { self.header() }
}
