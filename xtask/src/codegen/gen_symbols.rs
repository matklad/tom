//! This module generates `Symbol`s used by tom.

use crate::{project_root_dir, codegen, reformat};
use anyhow::Result;

pub fn gen_symbols(mode: codegen::Mode) -> Result<()> {
    let out_file = project_root_dir().join(codegen::SYMBOLS_OUT_FILE_PATH);
    codegen::verify_or_overwrite(mode, &out_file, &reformat(symbols_source_code())?)
}

fn symbols_source_code() -> String {
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
            r#"pub{} const {} = Symbol(unsafe {{ NonZeroU8::new_unchecked({}) }});"#,
            vis,
            name,
            i + 1
        )
    }
    buff
}
