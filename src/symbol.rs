use parse_tree::Symbol;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TomlSymbol(pub(crate) Symbol);

impl TomlSymbol {
    pub fn name(&self) -> &'static str {
        self.info().1
    }

    pub(crate) fn info(&self) -> &SymbolInfo {
        let idx = self.0;
        let idx = idx.0 as usize;
        &SYMBOLS[idx]
    }
}

impl fmt::Debug for TomlSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "`{}", self.name())
    }
}

pub(crate) struct SymbolInfo(u16, &'static str);

const SYMBOLS: &[SymbolInfo] = &[
    SymbolInfo(00, "WHITESPACE"),
    SymbolInfo(01, "DOC"),
    SymbolInfo(02, "KEY_VAL"),
    SymbolInfo(03, "ARRAY"),
    SymbolInfo(04, "DICT"),
    SymbolInfo(05, "TABLE_HEADER"),
    SymbolInfo(06, "TABLE"),
    SymbolInfo(07, "ARRAY_TABLE"),
    SymbolInfo(08, "EQ"),
    SymbolInfo(09, "DOT"),
    SymbolInfo(10, "COMMA"),
    SymbolInfo(11, "L_BRACK"),
    SymbolInfo(12, "R_BRACK"),
    SymbolInfo(13, "L_CURLY"),
    SymbolInfo(14, "R_CURLY"),
    SymbolInfo(15, "NUMBER"),
    SymbolInfo(16, "BOOL"),
    SymbolInfo(17, "BARE_KEY"),
    SymbolInfo(18, "BASIC_STRING"),
    SymbolInfo(19, "MULTILINE_BASIC_STRING"),
    SymbolInfo(20, "LITERAL_STRING"),
    SymbolInfo(21, "MULTILINE_LITERAL_STRING"),
    SymbolInfo(22, "DATE_TIME"),
    SymbolInfo(23, "ERROR"),
];

pub const WHITESPACE: TomlSymbol               = TomlSymbol(Symbol(SYMBOLS[00].0));
pub const DOC: TomlSymbol                     = TomlSymbol(Symbol(SYMBOLS[01].0));
pub const KEY_VAL: TomlSymbol                  = TomlSymbol(Symbol(SYMBOLS[02].0));
pub const ARRAY: TomlSymbol                    = TomlSymbol(Symbol(SYMBOLS[03].0));
pub const DICT: TomlSymbol                     = TomlSymbol(Symbol(SYMBOLS[04].0));
pub const TABLE_HEADER: TomlSymbol             = TomlSymbol(Symbol(SYMBOLS[05].0));
pub const TABLE: TomlSymbol                    = TomlSymbol(Symbol(SYMBOLS[06].0));
pub const ARRAY_TABLE: TomlSymbol              = TomlSymbol(Symbol(SYMBOLS[07].0));
pub const EQ: TomlSymbol                       = TomlSymbol(Symbol(SYMBOLS[08].0));
pub const DOT: TomlSymbol                      = TomlSymbol(Symbol(SYMBOLS[09].0));
pub const COMMA: TomlSymbol                    = TomlSymbol(Symbol(SYMBOLS[10].0));
pub const L_BRACK: TomlSymbol                  = TomlSymbol(Symbol(SYMBOLS[11].0));
pub const R_BRACK: TomlSymbol                  = TomlSymbol(Symbol(SYMBOLS[12].0));
pub const L_CURLY: TomlSymbol                  = TomlSymbol(Symbol(SYMBOLS[13].0));
pub const R_CURLY: TomlSymbol                  = TomlSymbol(Symbol(SYMBOLS[14].0));
pub const NUMBER: TomlSymbol                   = TomlSymbol(Symbol(SYMBOLS[15].0));
pub const BOOL: TomlSymbol                     = TomlSymbol(Symbol(SYMBOLS[16].0));
pub const BARE_KEY: TomlSymbol                 = TomlSymbol(Symbol(SYMBOLS[17].0));
pub const BASIC_STRING: TomlSymbol             = TomlSymbol(Symbol(SYMBOLS[18].0));
pub const MULTILINE_BASIC_STRING: TomlSymbol   = TomlSymbol(Symbol(SYMBOLS[19].0));
pub const LITERAL_STRING: TomlSymbol           = TomlSymbol(Symbol(SYMBOLS[20].0));
pub const MULTILINE_LITERAL_STRING: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[21].0));
pub const DATE_TIME: TomlSymbol                = TomlSymbol(Symbol(SYMBOLS[22].0));
pub const ERROR: TomlSymbol                    = TomlSymbol(Symbol(SYMBOLS[23].0));
