use super::{SymbolInfo, TomlSymbol, Symbol};

pub(crate) const SYMBOLS: &[SymbolInfo] = &[
    SymbolInfo(00, "WHITESPACE"),
    SymbolInfo(01, "COMMENT"),
    SymbolInfo(02, "DOC"),
    SymbolInfo(03, "ENTRY"),
    SymbolInfo(04, "KEY"),
    SymbolInfo(05, "VAL"),
    SymbolInfo(06, "ARRAY"),
    SymbolInfo(07, "DICT"),
    SymbolInfo(08, "TABLE_HEADER"),
    SymbolInfo(09, "TABLE"),
    SymbolInfo(10, "ARRAY_TABLE"),
    SymbolInfo(11, "EQ"),
    SymbolInfo(12, "DOT"),
    SymbolInfo(13, "COMMA"),
    SymbolInfo(14, "L_BRACK"),
    SymbolInfo(15, "R_BRACK"),
    SymbolInfo(16, "L_CURLY"),
    SymbolInfo(17, "R_CURLY"),
    SymbolInfo(18, "NUMBER"),
    SymbolInfo(19, "BOOL"),
    SymbolInfo(20, "BARE_KEY"),
    SymbolInfo(21, "BASIC_STRING"),
    SymbolInfo(22, "MULTILINE_BASIC_STRING"),
    SymbolInfo(23, "LITERAL_STRING"),
    SymbolInfo(24, "MULTILINE_LITERAL_STRING"),
    SymbolInfo(25, "DATE_TIME"),
    SymbolInfo(26, "ERROR"),
    SymbolInfo(27, "BARE_KEY_OR_NUMBER"),
    SymbolInfo(28, "BARE_KEY_OR_DATE"),
    SymbolInfo(29, "EOF"),
];

pub const WHITESPACE: TomlSymbol                     = TomlSymbol(Symbol(SYMBOLS[00].0));
pub const COMMENT: TomlSymbol                        = TomlSymbol(Symbol(SYMBOLS[01].0));
pub const DOC: TomlSymbol                            = TomlSymbol(Symbol(SYMBOLS[02].0));
pub const ENTRY: TomlSymbol                          = TomlSymbol(Symbol(SYMBOLS[03].0));
pub const KEY: TomlSymbol                            = TomlSymbol(Symbol(SYMBOLS[04].0));
pub const VAL: TomlSymbol                            = TomlSymbol(Symbol(SYMBOLS[05].0));
pub const ARRAY: TomlSymbol                          = TomlSymbol(Symbol(SYMBOLS[06].0));
pub const DICT: TomlSymbol                           = TomlSymbol(Symbol(SYMBOLS[07].0));
pub const TABLE_HEADER: TomlSymbol                   = TomlSymbol(Symbol(SYMBOLS[08].0));
pub const TABLE: TomlSymbol                          = TomlSymbol(Symbol(SYMBOLS[09].0));
pub const ARRAY_TABLE: TomlSymbol                    = TomlSymbol(Symbol(SYMBOLS[10].0));
pub const EQ: TomlSymbol                             = TomlSymbol(Symbol(SYMBOLS[11].0));
pub const DOT: TomlSymbol                            = TomlSymbol(Symbol(SYMBOLS[12].0));
pub const COMMA: TomlSymbol                          = TomlSymbol(Symbol(SYMBOLS[13].0));
pub const L_BRACK: TomlSymbol                        = TomlSymbol(Symbol(SYMBOLS[14].0));
pub const R_BRACK: TomlSymbol                        = TomlSymbol(Symbol(SYMBOLS[15].0));
pub const L_CURLY: TomlSymbol                        = TomlSymbol(Symbol(SYMBOLS[16].0));
pub const R_CURLY: TomlSymbol                        = TomlSymbol(Symbol(SYMBOLS[17].0));
pub const NUMBER: TomlSymbol                         = TomlSymbol(Symbol(SYMBOLS[18].0));
pub const BOOL: TomlSymbol                           = TomlSymbol(Symbol(SYMBOLS[19].0));
pub const BARE_KEY: TomlSymbol                       = TomlSymbol(Symbol(SYMBOLS[20].0));
pub const BASIC_STRING: TomlSymbol                   = TomlSymbol(Symbol(SYMBOLS[21].0));
pub const MULTILINE_BASIC_STRING: TomlSymbol         = TomlSymbol(Symbol(SYMBOLS[22].0));
pub const LITERAL_STRING: TomlSymbol                 = TomlSymbol(Symbol(SYMBOLS[23].0));
pub const MULTILINE_LITERAL_STRING: TomlSymbol       = TomlSymbol(Symbol(SYMBOLS[24].0));
pub const DATE_TIME: TomlSymbol                      = TomlSymbol(Symbol(SYMBOLS[25].0));
pub const ERROR: TomlSymbol                          = TomlSymbol(Symbol(SYMBOLS[26].0));
pub const BARE_KEY_OR_NUMBER: TomlSymbol             = TomlSymbol(Symbol(SYMBOLS[27].0));
pub const BARE_KEY_OR_DATE: TomlSymbol               = TomlSymbol(Symbol(SYMBOLS[28].0));
pub const EOF: TomlSymbol                            = TomlSymbol(Symbol(SYMBOLS[29].0));
