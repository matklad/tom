extern crate text_unit;
extern crate parse_tree as cst;
extern crate typed_arena;
extern crate itertools;
#[macro_use]
extern crate uncover;
extern crate m_lexer;
#[macro_use]
extern crate typed_index_derive;
#[macro_use]
extern crate lazy_static;

use std::{fmt, ptr, hash};

use cst::{ParseTree, PtNode, PtNodeId};

define_uncover_macros!(
    enable_if(cfg!(debug_assertions))
);

mod edit;
mod parser;
mod validator;
mod symbol;
mod visitor;

pub mod ast;
pub use text_unit::{TextRange, TextUnit};
pub use edit::{Edit, Position, Factory};
pub use symbol::TomlSymbol;

#[derive(Clone)]
pub struct TomlDoc {
    cst: ParseTree,
    parse_errors: Vec<SyntaxError>,
    text: String,
}

impl TomlDoc {
    pub fn new(text: String) -> TomlDoc {
        let (cst, errors) = parser::parse(&text);
        TomlDoc { cst, parse_errors: errors, text }
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        let mut res = self.parse_errors.clone();
        res.extend(validator::validate(self));
        res
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn cst(&self) -> CstNode {
        CstNode {
            doc: self,
            id: self.cst.root(),
        }
    }

    pub fn ast(&self) -> ast::Doc {
        ast::Doc::cast(self.cst()).unwrap()
    }

    pub fn edit(&self) -> Edit {
        Edit::new(self)
    }

    pub fn debug_dump(&self) -> String {
        let mut result = String::new();
        go(self.cst(), &mut result, 0);
        let errors = self.errors();
        if !errors.is_empty() {
            result += "\n";
            for e in errors.iter() {
                let text = &self.text()[e.range];
                result += &format!("error@{:?} {:?}: {}\n", e.range(), text, e.message());
            }
        }
        return result;

        fn go(node: CstNode, buff: &mut String, level: usize) {
            buff.push_str(&String::from("  ").repeat(level));
            buff.push_str(&format!("{:?}", node));

            if node.children().next().is_none() {
                let node_text = node.text();
                if !node_text.chars().all(char::is_whitespace) {
                    buff.push_str(&format!(" {:?}", node_text));
                }
            }
            buff.push('\n');
            for child in node.children() {
                go(child, buff, level + 1)
            }
        }
    }
}

impl fmt::Debug for TomlDoc {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "TomlDoc {{ ... }}")
    }
}

#[derive(Copy, Clone)]
pub struct CstNode<'f> {
    doc: &'f TomlDoc,
    id: PtNodeId,
}

impl<'f> PartialEq<CstNode<'f>> for CstNode<'f> {
    fn eq(&self, other: &CstNode) -> bool {
        self.id == other.id && ptr::eq(self.doc, other.doc)
    }
}

impl<'f> Eq for CstNode<'f> {}

impl<'f> hash::Hash for CstNode<'f> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<'t> fmt::Debug for CstNode<'t> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}@{:?}", self.symbol().name(), self.node().range())
    }
}

impl<'f> CstNode<'f> {
    pub fn doc(self) -> &'f TomlDoc {
        self.doc
    }

    pub fn symbol(self) -> TomlSymbol {
        TomlSymbol(self.node().symbol())
    }

    pub fn range(self) -> TextRange {
        self.node().range()
    }

    pub fn text(self) -> &'f str {
        &self.doc.text[self.range()]
    }

    pub fn parent(self) -> Option<CstNode<'f>> {
        self.node().parent().map(|id| CstNode {
            doc: self.doc,
            id,
        })
    }

    pub fn children(self) -> Children<'f> {
        Children {
            doc: self.doc,
            id: self.node().first_child(),
        }
    }

    pub fn is_leaf(self) -> bool {
        self.node().first_child().is_none()
    }

    fn node(self) -> &'f PtNode {
        &self.doc.cst[self.id]
    }
}

pub mod symbols {
    pub use symbol::{
        ERROR,

        WHITESPACE,
        COMMENT,

        DOC,
        ENTRY,
        KEY,
        VAL,
        ARRAY,
        DICT,
        TABLE_HEADER,
        TABLE,
        ARRAY_TABLE,

        EQ,
        DOT,
        COMMA,
        L_BRACK,
        R_BRACK,
        L_CURLY,
        R_CURLY,
        NUMBER,
        BOOL,
        BARE_KEY,
        BASIC_STRING,
        MULTILINE_BASIC_STRING,
        LITERAL_STRING,
        MULTILINE_LITERAL_STRING,
        DATE_TIME,
        BARE_KEY_OR_NUMBER,
        BARE_KEY_OR_DATE,
    };
}

#[derive(Clone)]
pub struct Children<'f> {
    doc: &'f TomlDoc,
    id: Option<PtNodeId>,
}

impl<'f> Iterator for Children<'f> {
    type Item = CstNode<'f>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.id.map(|id| {
            self.id = self.doc.cst[id].next_sibling();
            CstNode { doc: &self.doc, id }
        })
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    range: TextRange,
    message: String,
}

impl SyntaxError {
    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}
