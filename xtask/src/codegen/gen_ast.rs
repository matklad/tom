//! This module generates AST data types used by tom.

use heck::{CamelCase, ShoutySnakeCase};
use anyhow::Result;
use crate::{project_root_dir, codegen, reformat};

pub fn gen_ast(mode: codegen::Mode) -> Result<()> {
    let out_file = project_root_dir().join(codegen::AST_NODES_OUT_FILE_PATH);
    codegen::verify_or_overwrite(mode, &out_file, &reformat(ast_source_code())?)
}

fn descr() -> Vec<AstNode> {
    fn n(name: &'static str) -> AstNode {
        AstNode {
            name,
            symbols: Vec::new(),
            kinds: Vec::new(),
            methods: Vec::new(),
            text: false,
        }
    }

    vec![
        n("Doc")
            .methods(&["tables", "array_tables"])
            .method("entries", "Entry"),
        n("Table")
            .method("header", "TableHeader")
            .method("entries", "Entry"),
        n("ArrayTable")
            .method("header", "TableHeader")
            .method("entries", "Entry"),
        n("TableHeader").methods(&["keys"]),
        n("Entry").methods(&["keys", "value"]),
        n("Key").kinds(&["StringLit", "BareKey"]),
        n("Value").kinds(&["Array", "Dict", "Number", "Bool", "DateTime", "StringLit"]),
        n("StringLit")
            .symbols(&[
                "BASIC_STRING",
                "MULTILINE_BASIC_STRING",
                "LITERAL_STRING",
                "MULTILINE_LITERAL_STRING",
            ])
            .text(),
        n("BareKey").text(),
        n("Array").methods(&["values"]),
        n("Dict").method("entries", "Entry"),
        n("Number").text(),
        n("Bool").text(),
        n("DateTime").text(),
    ]
}

struct AstNode {
    name: &'static str,
    symbols: Vec<&'static str>,
    kinds: Vec<&'static str>,
    methods: Vec<Method>,
    text: bool,
}

impl AstNode {
    fn methods(mut self, names: &[&'static str]) -> AstNode {
        self.methods.extend(names.iter().map(|&name| {
            let type_name = if name.ends_with('s') {
                &name[..name.len() - 1]
            } else {
                name
            }
            .to_camel_case();
            Method {
                name,
                type_name,
                arity: if name.ends_with('s') {
                    Arity::Many
                } else {
                    Arity::One
                },
            }
        }));
        self
    }

    fn method(mut self, name: &'static str, type_name: &str) -> AstNode {
        let method = Method {
            name,
            type_name: type_name.to_owned(),
            arity: if name.ends_with('s') {
                Arity::Many
            } else {
                Arity::One
            },
        };
        self.methods.push(method);
        self
    }

    fn kinds(mut self, names: &[&'static str]) -> AstNode {
        self.kinds.extend(names.iter().copied());
        self
    }

    fn symbols(mut self, names: &[&'static str]) -> AstNode {
        self.symbols.extend(names.iter().copied());
        self
    }

    fn text(mut self) -> AstNode {
        self.text = true;
        self
    }
}

struct Method {
    name: &'static str,
    type_name: String,
    arity: Arity,
}

enum Arity {
    One,
    Many,
}

impl Method {
    fn ret_type(&self) -> String {
        match self.arity {
            Arity::One => format!("{}<'a>", self.type_name),
            Arity::Many => format!("AstChildren<'a, {}<'a>>", self.type_name),
        }
    }

    fn body(&self) -> &'static str {
        match self.arity {
            Arity::One => "AstChildren::new(self.syntax()).next().unwrap()",
            Arity::Many => "AstChildren::new(self.syntax())",
        }
    }
}

fn ast_source_code() -> String {
    let descr = descr();
    let mut buff = String::new();
    let mut nesting = 0;
    macro_rules! ln {
        () => { buff.push_str("\n") };
        ($($tt:tt)*) => {{
            let inner = format!($($tt)*);
            let mut indent = String::new();

            if inner == "}" || inner == "};" { nesting -= 1; }
            for _ in 0..nesting { indent += &"    " }
            if inner.ends_with("{") { nesting += 1; }

            buff.push_str(&indent);
            buff.push_str(&inner);
            buff.push_str("\n");
        }};
    }
    ln!("use crate::{{");
    ln!("SyntaxNode, SyntaxNodeRef, AstNode, AstChildren, TreeRoot, RefRoot, OwnedRoot, TomTypes,");
    ln!("symbol::*,");
    ln!("}};");
    ln!();

    for n in descr.iter() {
        ln!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        ln!(
            "pub struct {}Node<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);",
            n.name
        );
        ln!("pub type {}<'a> = {}Node<RefRoot<'a>>;", n.name, n.name);
        ln!();

        if !n.kinds.is_empty() {
            ln!("pub enum {}Kind<'a> {{", n.name);
            for k in n.kinds.iter() {
                ln!("{k}({k}<'a>),", k = k);
            }
            ln!("}}");
            ln!();
        }
    }

    for n in descr.iter() {
        ln!();
        ln!("impl<'a> AstNode<'a> for {}<'a> {{", n.name);
        {
            ln!("fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized {{ Self::cast(node) }}");
            ln!("fn syntax(self) -> SyntaxNodeRef<'a> {{ self.0 }}");
        }
        ln!("}}");
        ln!();

        ln!("impl<'a> From<{}<'a>> for SyntaxNodeRef<'a> {{", n.name);
        {
            ln!(
                "fn from(ast: {}<'a>) -> SyntaxNodeRef<'a> {{ ast.syntax() }}",
                n.name
            );
        }
        ln!("}}");
        ln!();

        ln!("impl<'a> {}<'a> {{", n.name);
        {
            ln!(
                "pub fn cast(node: SyntaxNodeRef<'a>) -> Option<{}> {{",
                n.name
            );
            {
                ln!("match node.symbol() {{");
                let symbols = if n.symbols.is_empty() {
                    vec![n.name.to_shouty_snake_case()]
                } else {
                    n.symbols.iter().map(|&s| s.to_string()).collect()
                };
                for s in symbols {
                    ln!("{} => Some({}Node(node)),", s, n.name);
                }
                ln!("_ => None,");
                ln!("}}");
            }
            ln!("}}");
            ln!();
            ln!("pub fn syntax(self) -> SyntaxNodeRef<'a> {{ self.0 }}");
            if !n.kinds.is_empty() || !n.methods.is_empty() || n.text {
                ln!();
            }

            if n.text {
                ln!("pub fn text(self) -> &'a str {{");
                ln!("self.syntax().leaf_text().unwrap()");
                ln!("}}");
            }

            if !n.kinds.is_empty() {
                ln!("pub fn kind(self) -> {}Kind<'a> {{", n.name);
                ln!("let node = self.syntax().children().next().unwrap();");
                for k in n.kinds.iter() {
                    ln!("if let Some(node) = {}::cast(node) {{", k);
                    ln!("return {}Kind::{}(node);", n.name, k);
                    ln!("}}");
                }
                ln!("unreachable!()");
                ln!("}}");
                ln!();
            }

            for m in n.methods.iter() {
                ln!("pub fn {}(self) -> {} {{", m.name, m.ret_type());
                ln!("{}", m.body());
                ln!("}}");
            }
        }
        ln!("}}");
    }
    buff
}
