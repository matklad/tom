extern crate lalrpop;
extern crate file;
extern crate heck;

use heck::ShoutySnakeCase;

fn main() {
    lalrpop::process_root().unwrap();
    gen_ast();
}

fn gen_ast() {
    let mut buff = String::new();
    macro_rules! ln {
        () => { buff.push_str("\n") };
        ($($tt:tt)*) => {{
            buff.push_str(&format!($($tt)*));
            buff.push_str("\n");
        }};
    }

    ln!("use parse_tree::Node;");
    ln!("use ast::{{AstNode, AstChildren}};");
    ln!("use symbols::*;");
    ln!();
    let wrappers = &[
        "File", "BareKey", "Array", "Dict", "Number", "Bool", "DateTime",
        "KeyVal", "Key", "Val", "Table", "ArrayTable", "TableHeader",
    ];
    let multi_wrappers = &[
        ("StringLit", &[
            "BASIC_STRING",
            "MULTILINE_BASIC_STRING",
            "LITERAL_STRING",
            "MULTILINE_LITERAL_STRING",
        ])
    ];

    for &symbol in wrappers.iter().chain(multi_wrappers.iter().map(|&(ref w, _)| w)) {
        ln!("#[derive(Clone, Copy, PartialEq, Eq)]");
        ln!("pub struct {}<'p>(Node<'p>);", symbol);
        ln!();
    }
    ln!();

    for &symbol in wrappers.iter() {
        ln!("impl<'p> AstNode<'p> for {}<'p> {{", symbol);
        ln!("    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {{");
        ln!(
            "        if node.symbol() == {} {{ Some({}(node)) }} else {{ None }}",
            symbol.to_shouty_snake_case(),
            symbol
        );
        ln!("    }}");
        ln!("    fn node(self) -> Node<'p> {{ self.0 }}");

        ln!("}}");
        ln!();
    }

    for &(ref symbol, ref m) in multi_wrappers.iter() {
        ln!("impl<'p> AstNode<'p> for {}<'p> {{", symbol);
        ln!("    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {{");
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
        ln!("    fn node(self) -> Node<'p> {{ self.0 }}");

        ln!("}}");
        ln!();
    }

    let methods: &[(&str, &[(&str, &str)])] = &[
        ("File", &[("tables", "Table"), ("array_tables", "ArrayTable")]),
        ("TableHeader", &[("keys", "Key")]),
        ("KeyVal", &[("key", "Key"), ("val", "Val")]),
    ];

    for &(ref s, ref ms) in methods.iter() {
        ln!("impl<'p> {}<'p> {{", s);
        for &(ref acc, ref s) in ms.iter() {
            let (ret, body) = if acc.ends_with("s") {
                (format!("AstChildren<'p, {}<'p>>", s), "AstChildren::new(self.node().children())")
            } else {
                (format!("{}<'p>", s), "AstChildren::new(self.node().children()).next().unwrap()")
            };
            ln!("    fn {}(&self) -> {} {{", acc, ret);
            ln!("        {}", body);
            ln!("    }}");
        }
        ln!("}}");
    }

    file::put_text("src/toml_ast.rs", &buff)
        .unwrap();
}
