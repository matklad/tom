extern crate clap;
extern crate failure;
extern crate heck;
extern crate itertools;

use clap::{App, Arg, SubCommand};
use itertools::Itertools;
use std::fs;
use std::process::exit;
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, failure::Error>;

mod gen_ast;

fn main() -> Result<()> {
    let gen_command = |name| {
        SubCommand::with_name(name).arg(
            Arg::with_name("verify")
                .long("--verify")
                .help("Verify that generated code is up-to-date"),
        )
    };
    let matches = App::new("tasks")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(gen_command("gen-ast"))
        .subcommand(gen_command("gen-symbols"))
        .subcommand(gen_command("gen-tests"))
        .subcommand(SubCommand::with_name("verify"))
        .get_matches();
    match matches.subcommand() {
        ("verify", _) => verify()?,
        (name, Some(matches)) => run_gen_command(name, matches.is_present("verify"))?,
        _ => unreachable!(),
    }
    Ok(())
}

fn run_gen_command(name: &str, verify: bool) -> Result<()> {
    match name {
        "gen-ast" => {
            update("./src/ast/generated.rs", &gen_ast::gen_ast(), verify)?;
        }
        "gen-symbols" => {
            update("./src/symbol/generated.rs", &gen_symbols(), verify)?;
        }
        "gen-tests" => get_tests(verify)?,
        _ => unreachable!(),
    };
    Ok(())
}

fn verify() -> Result<()> {
    run_gen_command("gen-ast", true)?;
    run_gen_command("gen-symbols", true)?;
    run_gen_command("gen-tests", true)?;
    Ok(())
}

fn update(path: &str, contents: &str, verify: bool) -> Result<()> {
    match fs::read_to_string(path) {
        Ok(ref old_contents) if old_contents == contents => {
            return Ok(());
        }
        _ => (),
    }
    if verify {
        eprintln!("error: `{}` is not up-to-date", path);
        exit(1);
    }
    fs::write(path, contents)?;
    Ok(())
}

fn get_tests(verify: bool) -> Result<()> {
    let src_dir = "./src/parser/grammar.rs";
    let grammar = fs::read_to_string(src_dir)?;
    let tests = collect_tests(&grammar);
    for (name, test) in &tests {
        let path = format!("./tests/data/inline/test-{}.toml", name);
        update(&path, test, verify)?;
    }
    // Check for strays.
    for file in fs::read_dir("tests/data/inline").unwrap() {
        let path = file.unwrap().path();
        let stem = path.file_stem().unwrap();
        let name = stem.to_str().unwrap();
        if name.starts_with("test-") {
            let name = name[5..].to_string();
            if !tests.contains_key(&name) {
                panic!(
                    "File `{}` exists, but no inline test found.",
                    path.display()
                );
            }
        }
    }
    return Ok(());

    fn collect_tests(s: &str) -> HashMap<String, String> {
        let mut res = HashMap::new();
        let comment_blocks = s
            .lines()
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

            let name = match block.next() {
                Some(line) if line.starts_with("test-") => line[5..].to_string(),
                _ => continue 'outer,
            };
            let text: String = itertools::join(block.chain(::std::iter::once("")), "\n");
            assert!(!text.trim().is_empty() && text.ends_with("\n"));
            if let Some(name) = res.insert(name, text) {
                panic!("Test name `{}` already used.", name);
            }
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
VALUE
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
FLOAT
INTEGER
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

    ln!("use super::{{SymbolInfo, Symbol, NonZeroU8}};");
    ln!();
    ln!("pub(crate) const SYMBOLS: &[SymbolInfo] = &[");
    for s in symbols.trim().lines() {
        ln!(r#"    SymbolInfo("{}"),"#, s)
    }
    ln!("];");
    ln!();
    for (i, s) in symbols.trim().lines().enumerate() {
        let name = format!("{}: Symbol", s);
        let vis = if name == "EOF" { "(crate)" } else { "" };
        ln!(
            r#"pub{} const {:<42} = Symbol(unsafe {{ NonZeroU8::new_unchecked({} + 1) }});"#,
            vis,
            name,
            i
        )
    }
    buff
}
