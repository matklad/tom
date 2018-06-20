use cst::Symbol;
use m_lexer::{LexerBuilder, Lexer, TokenKind};

use {
    TomlSymbol, TextUnit, TextRange,
    symbol,
};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Token {
    pub symbol: TomlSymbol,
    pub offset: TextUnit,
    pub len: TextUnit,
}

impl Token {
    pub fn is_significant(self) -> bool {
        match self.symbol {
            symbol::WHITESPACE | symbol::COMMENT => false,
            _ => true,
        }
    }

    pub fn range(self) -> TextRange {
        TextRange::offset_len(self.offset, self.len)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, TypedIndex)]
#[typed_index(Token)]
pub(crate) struct Pos(pub u32);

#[derive(Debug)]
pub(crate) struct Tokens {
    pub raw_tokens: Vec<Token>,
    pub significant: Vec<Pos>,
}

pub(crate) fn tokenize(input: &str) -> Tokens {
    let mut raw_tokens = Vec::new();
    let mut offset = TextUnit::from(0);
    for t in LEXER.tokenize(input) {
        let len = TextUnit::from(t.len as u32);
        raw_tokens.push(Token {
            symbol: TomlSymbol(Symbol(t.kind.0)),
            offset,
            len,
        });
        offset += len;
    }
    let significant = raw_tokens.iter()
        .enumerate()
        .filter(|(_idx, tok)| tok.is_significant())
        .map(|(idx, _tok)| Pos(idx as u32))
        .collect();
    Tokens { raw_tokens, significant }
}

lazy_static! {
    static ref LEXER: Lexer = lexer();
}


fn lexer() -> Lexer {
    fn t(s: TomlSymbol) -> TokenKind {
        TokenKind((s.0).0)
    }

    LexerBuilder::new()
        .error_token(t(symbol::ERROR))
        .tokens(&[
            (t(symbol::EQ), r"="),
            (t(symbol::DOT), r"\."),
            (t(symbol::COMMA), r","),
            (t(symbol::L_BRACK), r"\["),
            (t(symbol::R_BRACK), r"\]"),
            (t(symbol::L_CURLY), r"\{"),
            (t(symbol::R_CURLY), r"\}"),
            (
                t(symbol::WHITESPACE),
                r"\s+"
            ),
            (
                t(symbol::COMMENT),
                r"#.*"
            ),
            (
                t(symbol::BARE_KEY_OR_NUMBER),
                r"[0-9]+",
            ), (
                t(symbol::BARE_KEY_OR_DATE),
                r"[0-9]{4}-[0-9]{2}-[0-9]{2}[Zz]?",
            ), (
                t(symbol::BASIC_STRING),
                r#"(?x)
                    " ([^\r\n"\\] | \\.)* "
                "#,
            ), (
                t(symbol::MULTILINE_BASIC_STRING),
                r#"(?x)
                    """ ([^"] | \\. | "[^"] | ""[^"])* """
                "#,
            ), (
                t(symbol::LITERAL_STRING),
                r#"(?x)
                    ' ([^\r\n'] | \\.)* '
                "#,
            ), (
                t(symbol::MULTILINE_LITERAL_STRING),
                r#"(?x)
                    ''' ([^'] | \\. | '[^'] | ''[^'])* '''
                "#,
            ), (
                t(symbol::BOOL),
                r"(false|true)",
            ), (
                t(symbol::NUMBER),
                r"(?x)
                    [-+]?
                    (0|[1-9](_?[0-9])*) # no leading zeros
                    (\.[0-9](_?[0-9])*)?
                    ([eE][-+]?[1-9](_?[0-9])*)?
                ",
            ), (
                t(symbol::DATE_TIME),
                r"(?x)
                    ( [0-9]{4}-[0-9]{2}-[0-9]{2} ([Tt]([0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]+)?))?
                    | ([0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]+)?))
                    ([Zz]|[+-][0-9]{2}:[0-9]{2})?
                ",
            ), (
                t(symbol::BARE_KEY),
                r"[0-9_\-a-zA-Z]+",
            )
        ])
        .build()
}
