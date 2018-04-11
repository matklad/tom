use std::fmt;

/// A type of a syntactic construct, including both leaf tokens
/// and composite nodes, like "a comma" or "a table".
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Symbol(#[doc(hidden)] pub u32);

impl Symbol {
    pub fn name(&self) -> &'static str {
        SYMBOLS[self.0 as usize].1
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "`{}", self.name())
    }
}

struct SymbolInfo(u32, &'static str);

const SYMBOLS: &[SymbolInfo] = &[
    SymbolInfo(00, "WHITESPACE"),
    SymbolInfo(01, "FILE"),
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

pub const WHITESPACE: Symbol               = Symbol(SYMBOLS[00].0);
pub const FILE: Symbol                     = Symbol(SYMBOLS[01].0);
pub const KEY_VAL: Symbol                  = Symbol(SYMBOLS[02].0);
pub const ARRAY: Symbol                    = Symbol(SYMBOLS[03].0);
pub const DICT: Symbol                     = Symbol(SYMBOLS[04].0);
pub const TABLE_HEADER: Symbol             = Symbol(SYMBOLS[05].0);
pub const TABLE: Symbol                    = Symbol(SYMBOLS[06].0);
pub const ARRAY_TABLE: Symbol              = Symbol(SYMBOLS[07].0);
pub const EQ: Symbol                       = Symbol(SYMBOLS[08].0);
pub const DOT: Symbol                      = Symbol(SYMBOLS[09].0);
pub const COMMA: Symbol                    = Symbol(SYMBOLS[10].0);
pub const L_BRACK: Symbol                  = Symbol(SYMBOLS[11].0);
pub const R_BRACK: Symbol                  = Symbol(SYMBOLS[12].0);
pub const L_CURLY: Symbol                  = Symbol(SYMBOLS[13].0);
pub const R_CURLY: Symbol                  = Symbol(SYMBOLS[14].0);
pub const NUMBER: Symbol                   = Symbol(SYMBOLS[15].0);
pub const BOOL: Symbol                     = Symbol(SYMBOLS[16].0);
pub const BARE_KEY: Symbol                 = Symbol(SYMBOLS[17].0);
pub const BASIC_STRING: Symbol             = Symbol(SYMBOLS[18].0);
pub const MULTILINE_BASIC_STRING: Symbol   = Symbol(SYMBOLS[19].0);
pub const LITERAL_STRING: Symbol           = Symbol(SYMBOLS[20].0);
pub const MULTILINE_LITERAL_STRING: Symbol = Symbol(SYMBOLS[21].0);
pub const DATE_TIME: Symbol                = Symbol(SYMBOLS[22].0);
pub const ERROR: Symbol                    = Symbol(SYMBOLS[23].0);
