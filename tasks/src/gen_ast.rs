use heck::{ShoutySnakeCase, CamelCase};

fn descr() -> Vec<AstNode> {
    fn n(name: &'static str) -> AstNode {
        AstNode {
            name,
            symbols: Vec::new(),
            kinds: Vec::new(),
            methods: Vec::new(),
        }
    }

    vec![
        n("Doc").methods(&["tables", "array_tables"]).method("entries", "Entry"),
        n("Table").method("header", "TableHeader").method("entries", "Entry"),
        n("ArrayTable").method("header", "TableHeader").method("entries", "Entry"),
        n("TableHeader").methods(&["keys"]),
        n("Entry").methods(&["keys", "val"]),
        n("Key").kinds(&["StringLit", "BareKey"]),
        n("Val").kinds(&["Array", "Dict", "Number", "Bool", "DateTime", "StringLit"]),
        n("StringLit").symbols(&["BASIC_STRING", "MULTILINE_BASIC_STRING", "LITERAL_STRING", "MULTILINE_LITERAL_STRING"]),
        n("BareKey"),
        n("Array"),
        n("Dict").method("entries", "Entry"),
        n("Number"),
        n("Bool"),
        n("DateTime"),
    ]
}

struct AstNode {
    name: &'static str,
    symbols: Vec<&'static str>,
    kinds: Vec<&'static str>,
    methods: Vec<Method>,
}

impl AstNode {
    fn methods(mut self, names: &[&'static str]) -> AstNode {
        self.methods.extend(names.iter().map(|&name| {
            let type_name = if name.ends_with("s") {
                &name[..name.len() - 1]
            } else {
                name
            }.to_camel_case();
            Method {
                name,
                type_name,
                arity: if name.ends_with("s") { Arity::Many } else { Arity::One },
            }
        }));
        self
    }

    fn method(mut self, name: &'static str, type_name: &str) -> AstNode {
        let method = Method {
            name,
            type_name: type_name.to_owned(),
            arity: if name.ends_with("s") { Arity::Many } else { Arity::One },
        };
        self.methods.push(method);
        self
    }

    fn kinds(mut self, names: &[&'static str]) -> AstNode {
        self.kinds.extend(names.iter().map(|&name| name));
        self
    }

    fn symbols(mut self, names: &[&'static str]) -> AstNode {
        self.symbols.extend(names.iter().map(|&name| name));
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
            Arity::One => format!("{}<'f>", self.type_name),
            Arity::Many => format!("AstChildren<'f, {}<'f>>", self.type_name),
        }
    }

    fn body(&self) -> &'static str {
        match self.arity {
            Arity::One =>
                "AstChildren::new(self.cst().children()).next().unwrap()",
            Arity::Many =>
                "AstChildren::new(self.cst().children())",
        }
    }
}

pub fn gen_ast() -> String {
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
    ln!("use {{");
    ln!("CstNode,");
    ln!("ast::{{AstNode, AstChildren}},");
    ln!("symbols::*,");
    ln!("}};");
    ln!();

    for n in descr.iter() {
        ln!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        ln!("pub struct {}<'f>(CstNode<'f>);", n.name);
        ln!();

        if !n.kinds.is_empty() {
            ln!("pub enum {}Kind<'f> {{", n.name);
            for k in n.kinds.iter() {
                ln!("{}({}<'f>),", k, k);
            }
            ln!("}}");
            ln!();
        }
    }

    for n in descr.iter() {
        ln!();
        ln!("impl<'f> AstNode<'f> for {}<'f> {{", n.name);
        {
            ln!("fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized {{ Self::cast(node) }}");
            ln!("fn cst(self) -> CstNode<'f> {{ self.cst() }}");
        }
        ln!("}}");
        ln!();

        ln!("impl<'f> From<{}<'f>> for CstNode<'f> {{", n.name);
        {
            ln!("fn from(ast: {}<'f>) -> CstNode<'f> {{ ast.cst() }}", n.name);
        }
        ln!("}}");
        ln!();

        ln!("impl<'f> {}<'f> {{", n.name);
        {
            ln!("pub fn cast(node: CstNode<'f>) -> Option<{}<'f>> {{", n.name);
            {
                ln!("match node.symbol() {{");
                let symbols = if n.symbols.is_empty() {
                    vec![n.name.to_shouty_snake_case()]
                } else {
                    n.symbols.iter().map(|s| s.to_string()).collect()
                };
                for s in symbols {
                    ln!("{} => Some({}(node)),", s, n.name);
                }
                ln!("_ => None,");
                ln!("}}");
            }
            ln!("}}");
            ln!();
            ln!("pub fn cst(self) -> CstNode<'f> {{ self.0 }}");
            if !n.kinds.is_empty() || !n.methods.is_empty() {
                ln!();
            }

            if !n.kinds.is_empty() {
                ln!("pub fn kind(self) -> {}Kind<'f> {{", n.name);
                ln!("let node = self.cst().children().next().unwrap();");
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
