extern crate heck;
extern crate clap;
extern crate itertools;
extern crate failure;

use itertools::Itertools;
use clap::{App, SubCommand};
use heck::{ShoutySnakeCase, CamelCase};
use std::fs;

type Result<T> = ::std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let matches = App::new("tasks")
        .subcommand(SubCommand::with_name("gen-ast"))
        .subcommand(SubCommand::with_name("gen-symbols"))
        .subcommand(SubCommand::with_name("gen-tests"))
        .get_matches();
    match matches.subcommand_name().unwrap() {
        "gen-ast" => {
            update(
                "./src/ast/generated.rs",
                &gen_ast(),
            )?;
        }
        "gen-symbols" => {
            update(
                "./src/symbol/generated.rs",
                &gen_symbols(),
            )?;
        }
        "gen-tests" => get_tests()?,
        _ => unreachable!()
    };
    Ok(())
}

fn update(path: &str, contents: &str) -> Result<()> {
    match fs::read_to_string(path) {
        Ok(ref old_contents) if old_contents == contents => {
            return Ok(());
        }
        _ => (),
    }
    fs::write(path, contents)?;
    Ok(())
}

fn get_tests() -> Result<()> {
    let src_dir = "./src/parser/rd/grammar.rs";
    let grammar = fs::read_to_string(src_dir)?;
    let tests = collect_tests(&grammar);
    for (i, test) in tests.into_iter().enumerate() {
        let path = format!("./tests/data/inline/test_{:02}.toml", i);
        update(&path, &test)?;
    }
    return Ok(());

    fn collect_tests(s: &str) -> Vec<String> {
        let mut res = vec![];
        let comment_blocks = s.lines()
            .map(str::trim_left)
            .group_by(|line| line.starts_with("//"));

        'outer: for (is_comment, block) in comment_blocks.into_iter() {
            if !is_comment {
                continue;
            }
            let mut block = block.map(|line| {
                let prefix = if line.starts_with("// ") { "// " } else { "//" };
                &line[prefix.len()..]
            });

            match block.next() {
                Some(line) if line.starts_with("test") => (),
                _ => continue 'outer,
            }
            let text: String = itertools::join(block.chain(::std::iter::once("")), "\n");
            assert!(!text.trim().is_empty() && text.ends_with("\n"));
            res.push(text)
        }
        res
    }
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
            Method {
                name,
                arity: if name.ends_with("s") { Arity::Many } else { Arity::One },
            }
        }));
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
    arity: Arity,
}
enum Arity {
    One,
    Many,
}

impl Method {
    fn ret_type(&self) -> String {
        match self.arity {
            Arity::One => format!("{}<'f>", self.name.to_camel_case()),
            Arity::Many => format!("AstChildren<'f, {}<'f>>", self.name[..self.name.len() - 1].to_camel_case()),
        }
    }

    fn body(&self) -> &'static str {
        match self.arity {
            Arity::One =>
                "AstChildren::new(self.node().children()).next().unwrap()",
            Arity::Many =>
                "AstChildren::new(self.node().children())",
        }
    }
}


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
        n("Doc").methods(&["tables", "array_tables"]),
        n("Table"),
        n("ArrayTable"),
        n("TableHeader").methods(&["keys"]),
        n("KeyVal").methods(&["key", "val"]),
        n("Key").kinds(&["StringLit", "BareKey"]),
        n("Val").kinds(&["Array", "Dict", "Number", "Bool", "DateTime", "StringLit"]),
        n("StringLit").symbols(&["BASIC_STRING", "MULTILINE_BASIC_STRING", "LITERAL_STRING", "MULTILINE_LITERAL_STRING"]),
        n("BareKey"),
        n("Array"),
        n("Dict"),
        n("Number"),
        n("Bool"),
        n("DateTime"),
    ]
}


fn gen_ast() -> String {
    let descr = descr();
    let mut buff = String::new();
    let mut nesting = 0;
    macro_rules! ln {
        () => { buff.push_str("\n") };
        ($($tt:tt)*) => {{
            let inner = format!($($tt)*);
            let mut indent = String::new();

            if inner == "}" { nesting -= 1; }
            for _ in 0..nesting { indent += &"    " }
            if inner.ends_with("{") { nesting += 1; }

            buff.push_str(&indent);
            buff.push_str(&inner);
            buff.push_str("\n");
        }};
    }

    ln!("use *;");
    ln!("use ast::{{AstNode, AstChildren}};");
    ln!();

    for n in descr.iter() {
        ln!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        ln!("pub struct {}<'f>(TomlNode<'f>);", n.name);
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
            ln!("fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {{");
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
            ln!("fn node(self) -> TomlNode<'f> {{ self.0 }}");
        }
        ln!("}}");
        ln!();

        ln!("impl<'f> From<{}<'f>> for TomlNode<'f> {{", n.name);
        {
            ln!("fn from(ast: {}<'f>) -> TomlNode<'f> {{ ast.node() }}", n.name);
        }
        ln!("}}");
        ln!();

        ln!("impl<'f> {}<'f> {{", n.name);
        {
            ln!("pub fn node(self) -> TomlNode<'f> {{ self.0 }}");
            if !n.kinds.is_empty() || !n.methods.is_empty() {
                ln!();
            }

            if !n.kinds.is_empty() {
                ln!("pub fn kind(self) -> {}Kind<'f> {{", n.name);
                ln!("let node = self.node().children().next().unwrap();");
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

fn gen_symbols() -> String {
    let mut buff = String::new();
    macro_rules! ln {
        () => { buff.push_str("\n") };
        ($($tt:tt)*) => {{
            buff.push_str(&format!($($tt)*));
            buff.push_str("\n");
        }};
    }
    let symbols = "
WHITESPACE
COMMENT
DOC
KEY_VAL
KEY
VAL
ARRAY
DICT
TABLE_HEADER
TABLE
ARRAY_TABLE
EQ
DOT
COMMA
L_BRACK
R_BRACK
L_CURLY
R_CURLY
NUMBER
BOOL
BARE_KEY
BASIC_STRING
MULTILINE_BASIC_STRING
LITERAL_STRING
MULTILINE_LITERAL_STRING
DATE_TIME
ERROR
BARE_KEY_OR_NUMBER
BARE_KEY_OR_DATE
EOF
";

    ln!("use super::{{SymbolInfo, TomlSymbol, Symbol}};");
    ln!();
    ln!("pub(crate) const SYMBOLS: &[SymbolInfo] = &[");
    for (i, s) in symbols.trim().lines().enumerate() {
        ln!(r#"    SymbolInfo({:02}, "{}"),"#, i, s)
    }
    ln!("];");
    ln!();
    for (i, s) in symbols.trim().lines().enumerate() {
        let name = format!("{}: TomlSymbol", s);
        ln!(r#"pub const {:<20} = TomlSymbol(Symbol(SYMBOLS[{:02}].0));"#, name, i)
    }
    buff
}
