extern crate heck;
extern crate clap;
extern crate itertools;
extern crate failure;

use itertools::Itertools;
use clap::{App, SubCommand};
use heck::ShoutySnakeCase;
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
DOC
KEY_VAL
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
