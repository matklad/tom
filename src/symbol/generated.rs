use super::{SymbolInfo, Symbol, NonZeroU8};

pub(crate) const SYMBOLS: &[SymbolInfo] = &[
    SymbolInfo("ERROR"),
    SymbolInfo("WHITESPACE"),
    SymbolInfo("COMMENT"),
    SymbolInfo("DOC"),
    SymbolInfo("ENTRY"),
    SymbolInfo("KEY"),
    SymbolInfo("VALUE"),
    SymbolInfo("ARRAY"),
    SymbolInfo("DICT"),
    SymbolInfo("TABLE_HEADER"),
    SymbolInfo("TABLE"),
    SymbolInfo("ARRAY_TABLE"),
    SymbolInfo("EQ"),
    SymbolInfo("DOT"),
    SymbolInfo("COMMA"),
    SymbolInfo("L_BRACK"),
    SymbolInfo("R_BRACK"),
    SymbolInfo("L_CURLY"),
    SymbolInfo("R_CURLY"),
    SymbolInfo("NUMBER"),
    SymbolInfo("BOOL"),
    SymbolInfo("BARE_KEY"),
    SymbolInfo("BASIC_STRING"),
    SymbolInfo("MULTILINE_BASIC_STRING"),
    SymbolInfo("LITERAL_STRING"),
    SymbolInfo("MULTILINE_LITERAL_STRING"),
    SymbolInfo("DATE_TIME"),
    SymbolInfo("BARE_KEY_OR_NUMBER"),
    SymbolInfo("BARE_KEY_OR_DATE"),
    SymbolInfo("EOF"),
];

pub const ERROR: Symbol                              = Symbol(unsafe { NonZeroU8::new_unchecked(0 + 1) });
pub const WHITESPACE: Symbol                         = Symbol(unsafe { NonZeroU8::new_unchecked(1 + 1) });
pub const COMMENT: Symbol                            = Symbol(unsafe { NonZeroU8::new_unchecked(2 + 1) });
pub const DOC: Symbol                                = Symbol(unsafe { NonZeroU8::new_unchecked(3 + 1) });
pub const ENTRY: Symbol                              = Symbol(unsafe { NonZeroU8::new_unchecked(4 + 1) });
pub const KEY: Symbol                                = Symbol(unsafe { NonZeroU8::new_unchecked(5 + 1) });
pub const VALUE: Symbol                              = Symbol(unsafe { NonZeroU8::new_unchecked(6 + 1) });
pub const ARRAY: Symbol                              = Symbol(unsafe { NonZeroU8::new_unchecked(7 + 1) });
pub const DICT: Symbol                               = Symbol(unsafe { NonZeroU8::new_unchecked(8 + 1) });
pub const TABLE_HEADER: Symbol                       = Symbol(unsafe { NonZeroU8::new_unchecked(9 + 1) });
pub const TABLE: Symbol                              = Symbol(unsafe { NonZeroU8::new_unchecked(10 + 1) });
pub const ARRAY_TABLE: Symbol                        = Symbol(unsafe { NonZeroU8::new_unchecked(11 + 1) });
pub const EQ: Symbol                                 = Symbol(unsafe { NonZeroU8::new_unchecked(12 + 1) });
pub const DOT: Symbol                                = Symbol(unsafe { NonZeroU8::new_unchecked(13 + 1) });
pub const COMMA: Symbol                              = Symbol(unsafe { NonZeroU8::new_unchecked(14 + 1) });
pub const L_BRACK: Symbol                            = Symbol(unsafe { NonZeroU8::new_unchecked(15 + 1) });
pub const R_BRACK: Symbol                            = Symbol(unsafe { NonZeroU8::new_unchecked(16 + 1) });
pub const L_CURLY: Symbol                            = Symbol(unsafe { NonZeroU8::new_unchecked(17 + 1) });
pub const R_CURLY: Symbol                            = Symbol(unsafe { NonZeroU8::new_unchecked(18 + 1) });
pub const NUMBER: Symbol                             = Symbol(unsafe { NonZeroU8::new_unchecked(19 + 1) });
pub const BOOL: Symbol                               = Symbol(unsafe { NonZeroU8::new_unchecked(20 + 1) });
pub const BARE_KEY: Symbol                           = Symbol(unsafe { NonZeroU8::new_unchecked(21 + 1) });
pub const BASIC_STRING: Symbol                       = Symbol(unsafe { NonZeroU8::new_unchecked(22 + 1) });
pub const MULTILINE_BASIC_STRING: Symbol             = Symbol(unsafe { NonZeroU8::new_unchecked(23 + 1) });
pub const LITERAL_STRING: Symbol                     = Symbol(unsafe { NonZeroU8::new_unchecked(24 + 1) });
pub const MULTILINE_LITERAL_STRING: Symbol           = Symbol(unsafe { NonZeroU8::new_unchecked(25 + 1) });
pub const DATE_TIME: Symbol                          = Symbol(unsafe { NonZeroU8::new_unchecked(26 + 1) });
pub const BARE_KEY_OR_NUMBER: Symbol                 = Symbol(unsafe { NonZeroU8::new_unchecked(27 + 1) });
pub const BARE_KEY_OR_DATE: Symbol                   = Symbol(unsafe { NonZeroU8::new_unchecked(28 + 1) });
pub const EOF: Symbol                                = Symbol(unsafe { NonZeroU8::new_unchecked(29 + 1) });
