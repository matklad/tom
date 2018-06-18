use m_lexer::{LexerBuilder, Lexer, TokenKind};

use {
    TomlSymbol, TextUnit,
    symbol,
};

struct Token {
    symbol: TomlSymbol,
    len: TextUnit,
}

struct Tokens {
    all: Vec<Token>,
//    significant: Vec<Pos>,
}

fn tokenize(input: &str) -> Tokens {
    unimplemented!()
}


fn lexer() -> Lexer {
    fn t(s: TomlSymbol) -> TokenKind {
        TokenKind((s.0).0)
    }

    LexerBuilder::new()
        .error_token(t(symbol::ERROR))
        .tokens(&[
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
