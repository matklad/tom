extern crate heck;
extern crate clap;
extern crate itertools;
extern crate failure;

use itertools::Itertools;
use clap::{App, SubCommand};
use std::fs;

type Result<T> = ::std::result::Result<T, failure::Error>;

mod gen_ast;

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
                &gen_ast::gen_ast(),
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
    let src_dir = "./src/parser/grammar.rs";
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
ERROR
WHITESPACE
COMMENT
DOC
ENTRY
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
        let vis = if name == "EOF" { "(crate)" } else { "" };
        ln!(r#"pub{} const {:<42} = TomlSymbol(Symbol(SYMBOLS[{:02}].0));"#, vis, name, i)
    }
    buff
}
