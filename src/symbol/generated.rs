use super::{SymbolInfo, TomlSymbol, Symbol};

pub(crate) const SYMBOLS: &[SymbolInfo] = &[
    SymbolInfo(00, "WHITESPACE"),
    SymbolInfo(01, "COMMENT"),
    SymbolInfo(02, "DOC"),
    SymbolInfo(03, "KEY_VAL"),
    SymbolInfo(04, "ARRAY"),
    SymbolInfo(05, "DICT"),
    SymbolInfo(06, "TABLE_HEADER"),
    SymbolInfo(07, "TABLE"),
    SymbolInfo(08, "ARRAY_TABLE"),
    SymbolInfo(09, "EQ"),
    SymbolInfo(10, "DOT"),
    SymbolInfo(11, "COMMA"),
    SymbolInfo(12, "L_BRACK"),
    SymbolInfo(13, "R_BRACK"),
    SymbolInfo(14, "L_CURLY"),
    SymbolInfo(15, "R_CURLY"),
    SymbolInfo(16, "NUMBER"),
    SymbolInfo(17, "BOOL"),
    SymbolInfo(18, "BARE_KEY"),
    SymbolInfo(19, "BASIC_STRING"),
    SymbolInfo(20, "MULTILINE_BASIC_STRING"),
    SymbolInfo(21, "LITERAL_STRING"),
    SymbolInfo(22, "MULTILINE_LITERAL_STRING"),
    SymbolInfo(23, "DATE_TIME"),
    SymbolInfo(24, "ERROR"),
    SymbolInfo(25, "BARE_KEY_OR_NUMBER"),
    SymbolInfo(26, "BARE_KEY_OR_DATE"),
    SymbolInfo(27, "EOF"),
];

pub const WHITESPACE: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[00].0));
pub const COMMENT: TomlSymbol  = TomlSymbol(Symbol(SYMBOLS[01].0));
pub const DOC: TomlSymbol      = TomlSymbol(Symbol(SYMBOLS[02].0));
pub const KEY_VAL: TomlSymbol  = TomlSymbol(Symbol(SYMBOLS[03].0));
pub const ARRAY: TomlSymbol    = TomlSymbol(Symbol(SYMBOLS[04].0));
pub const DICT: TomlSymbol     = TomlSymbol(Symbol(SYMBOLS[05].0));
pub const TABLE_HEADER: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[06].0));
pub const TABLE: TomlSymbol    = TomlSymbol(Symbol(SYMBOLS[07].0));
pub const ARRAY_TABLE: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[08].0));
pub const EQ: TomlSymbol       = TomlSymbol(Symbol(SYMBOLS[09].0));
pub const DOT: TomlSymbol      = TomlSymbol(Symbol(SYMBOLS[10].0));
pub const COMMA: TomlSymbol    = TomlSymbol(Symbol(SYMBOLS[11].0));
pub const L_BRACK: TomlSymbol  = TomlSymbol(Symbol(SYMBOLS[12].0));
pub const R_BRACK: TomlSymbol  = TomlSymbol(Symbol(SYMBOLS[13].0));
pub const L_CURLY: TomlSymbol  = TomlSymbol(Symbol(SYMBOLS[14].0));
pub const R_CURLY: TomlSymbol  = TomlSymbol(Symbol(SYMBOLS[15].0));
pub const NUMBER: TomlSymbol   = TomlSymbol(Symbol(SYMBOLS[16].0));
pub const BOOL: TomlSymbol     = TomlSymbol(Symbol(SYMBOLS[17].0));
pub const BARE_KEY: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[18].0));
pub const BASIC_STRING: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[19].0));
pub const MULTILINE_BASIC_STRING: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[20].0));
pub const LITERAL_STRING: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[21].0));
pub const MULTILINE_LITERAL_STRING: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[22].0));
pub const DATE_TIME: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[23].0));
pub const ERROR: TomlSymbol    = TomlSymbol(Symbol(SYMBOLS[24].0));
pub const BARE_KEY_OR_NUMBER: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[25].0));
pub const BARE_KEY_OR_DATE: TomlSymbol = TomlSymbol(Symbol(SYMBOLS[26].0));
pub const EOF: TomlSymbol      = TomlSymbol(Symbol(SYMBOLS[27].0));
