extern crate heck;
extern crate lalrpop;

use heck::ShoutySnakeCase;
use std::fs;

fn main() {
    lalrpop::process_root().unwrap();

    let ast = gen_ast();

    let generated_ast = "src/ast/generated.rs";
    match fs::read_to_string(generated_ast) {
        Ok(ref old_ast) if old_ast == &ast => {
            return;
        }
        _ => (),
    }
    fs::write(generated_ast, &ast).unwrap();
}

fn gen_ast() -> String {
    let mut buff = String::new();
    macro_rules! ln {
        () => { buff.push_str("\n") };
        ($($tt:tt)*) => {{
            buff.push_str(&format!($($tt)*));
            buff.push_str("\n");
        }};
    }

    ln!("use *;");
    ln!("use ast::{{AstNode, AstChildren}};");
    ln!();
    let wrappers = &[
        "Doc",
        "BareKey",
        "Array",
        "Dict",
        "Number",
        "Bool",
        "DateTime",
        "KeyVal",
        "Table",
        "ArrayTable",
        "TableHeader",
    ];
    let multi_wrappers = &[
        (
            "StringLit",
            &[
                "BASIC_STRING",
                "MULTILINE_BASIC_STRING",
                "LITERAL_STRING",
                "MULTILINE_LITERAL_STRING",
            ],
        ),
    ];
    let enums: &[(&str, &[&str])] = &[
        ("Key", &["StringLit", "BareKey"]),
        (
            "Val",
            &["Array", "Dict", "Number", "Bool", "DateTime", "StringLit"],
        ),
    ];

    for &symbol in wrappers
        .iter()
        .chain(multi_wrappers.iter().map(|&(ref w, _)| w))
    {
        ln!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        ln!("pub struct {}<'f>(TomlNode<'f>);", symbol);
        ln!();
    }
    ln!();

    for &(ref symbol, ref variants) in enums.iter() {
        ln!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        ln!("pub enum {}<'f> {{", symbol);
        for &v in variants.iter() {
            ln!("    {}({}<'f>),", v, v);
        }
        ln!("}}");
        ln!();
    }

    for &symbol in wrappers.iter() {
        ln!("impl<'f> AstNode<'f> for {}<'f> {{", symbol);
        ln!("    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {{");
        ln!(
            "        if node.symbol() == {} {{ Some({}(node)) }} else {{ None }}",
            symbol.to_shouty_snake_case(),
            symbol
        );
        ln!("    }}");
        ln!("    fn node(self) -> TomlNode<'f> {{ self.0 }}");

        ln!("}}");
        ln!();
    }

    for &(ref symbol, ref m) in multi_wrappers.iter() {
        ln!("impl<'f> AstNode<'f> for {}<'f> {{", symbol);
        ln!("    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {{");
        ln!("        match node.symbol() {{");
        for &s in m.iter() {
            ln!(
                "            {} => Some({}(node)),",
                s.to_shouty_snake_case(),
                symbol
            );
        }
        ln!("            _ => None,");
        ln!("        }}");
        ln!("    }}");
        ln!("    fn node(self) -> TomlNode<'f> {{ self.0 }}");

        ln!("}}");
        ln!();
    }

    for &(ref symbol, ref variants) in enums.iter() {
        ln!("impl<'f> AstNode<'f> for {}<'f> {{", symbol);
        ln!("    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {{");
        for &v in variants.iter() {
            ln!(
                "        if let Some(n) = {}::cast(node) {{ return Some({}::{}(n)); }}",
                v,
                symbol,
                v,
            );
        }
        ln!("        None");
        ln!("    }}");
        ln!("    fn node(self) -> TomlNode<'f> {{");
        ln!("        match self {{");
        for &v in variants.iter() {
            ln!("            {}::{}(n) => n.node(),", symbol, v);
        }
        ln!("        }}");
        ln!("    }}");
        ln!("}}");
        ln!();
    }

    let all_symbols =
        wrappers.iter()
            .chain(multi_wrappers.iter().map(|(s, _)| s))
            .chain(enums.iter().map(|(s, _)| s));
    for symbol in all_symbols {
        ln!("impl<'f> From<{}<'f>> for TomlNode<'f> {{", symbol);
        ln!("    fn from(ast: {}<'f>) -> TomlNode<'f> {{ ast.node() }}", symbol);
        ln!("}}");
        ln!();
    }

    let methods: &[(&str, &[(&str, &str)])] = &[
        (
            "Doc",
            &[("tables", "Table"), ("array_tables", "ArrayTable")],
        ),
        ("TableHeader", &[("keys", "Key")]),
        ("KeyVal", &[("key", "Key"), ("val", "Val")]),
    ];

    for &(ref s, ref ms) in methods.iter() {
        ln!("impl<'f> {}<'f> {{", s);
        for &(ref acc, ref s) in ms.iter() {
            let (ret, body) = if acc.ends_with("s") {
                (
                    format!("AstChildren<'f, {}<'f>>", s),
                    "AstChildren::new(self.node().children())",
                )
            } else {
                (
                    format!("{}<'f>", s),
                    "AstChildren::new(self.node().children()).next().unwrap()",
                )
            };
            ln!("    pub fn {}(self) -> {} {{", acc, ret);
            ln!("        {}", body);
            ln!("    }}");
        }
        ln!("}}");
    }
    buff
}
