use parse_tree::Symbol;
use m_lexer::{self, LexerBuilder, Lexer, TokenKind};

use {
    TomlSymbol, TextUnit,
    symbol,
};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Token {
    pub symbol: TomlSymbol,
    pub len: TextUnit,
}

impl Token {
    fn from_m_lexer(t: m_lexer::Token) -> Self {
        Token {
            symbol: TomlSymbol(Symbol(t.kind.0)),
            len: TextUnit::from(t.len as u32),
        }
    }

    pub fn is_significant(self) -> bool {
        match self.symbol {
            symbol::WHITESPACE => false,
            _ => true,
        }
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
    let raw_tokens = LEXER.tokenize(input)
        .into_iter()
        .map(Token::from_m_lexer)
        .collect::<Vec<_>>();
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
