use parse_tree::{TopDownBuilder, ParseTree};
use {
    TomlSymbol, SyntaxError,
    symbol::{DOC, KEY_VAL, TABLE, COMMENT, WHITESPACE},
};

mod lexer;
mod grammar;

pub(crate) fn parse(input: &str) -> (ParseTree, Vec<SyntaxError>) {
    let tokens = lexer::tokenize(input);
    let mut sink = EventSink::new(input, &tokens);
    {
        let mut parser = Parser {
            sink: &mut sink,
            tokens: &tokens,
            pos: 0,
        };
        parser.parse();
    }
    (sink.builder.finish(), sink.errors)
}

struct Parser<'s, 't: 's> {
    sink: &'s mut EventSink<'t>,
    tokens: &'t lexer::Tokens,
    pos: usize,
}

struct EventSink<'t> {
    pos: lexer::Pos,
    text: &'t str,
    tokens: &'t lexer::Tokens,
    builder: TopDownBuilder,
    errors: Vec<SyntaxError>,
}

impl<'t> EventSink<'t> {
    fn new(text: &'t str, tokens: &'t lexer::Tokens) -> Self {
        EventSink {
            pos: lexer::Pos(0),
            text,
            tokens,
            builder: TopDownBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn start(&mut self, s: TomlSymbol) {
//        eprintln!("start {:?} at {:?}", s, pos);
        let ws = self.whitespace();
        let n = self.leading_ws(ws, s);
        for _ in 0..(ws.len() - n) {
            self.bump(None)
        }
        self.builder.start_internal(s.0);
    }

    fn finish(&mut self, s: TomlSymbol) {
//        eprintln!("finished {:?} at {:?}", s, self.last_consumed);
        let ws = self.whitespace();
        let n = self.trailing_ws(ws, s);
        for _ in 0..n {
            self.bump(None)
        }
        self.builder.finish_internal();
    }

    fn token(&mut self, pos: lexer::Pos, s: Option<TomlSymbol>) {
        while self.pos < pos {
            self.bump(None)
        }
        self.bump(s);
    }

    fn error(&mut self, message: impl Into<String>) {
        let mut tok = &self.tokens.raw_tokens[self.pos - 1];
        let mut pos = self.pos;
        loop {
            match pos.get(&self.tokens.raw_tokens) {
                Some(t) if t.is_significant() => {
                    tok = t;
                    break;
                }
                Some(t) => {
                    tok = t;
                }
                None => break,
            }
            pos += 1;
        }

        self.errors.push(SyntaxError {
            range: tok.range(),
            message: message.into(),
        })
    }

    fn leading_ws(&self, ws: &[lexer::Token], s: TomlSymbol) -> usize {
        match s {
            DOC => ws.len(),
            KEY_VAL | TABLE => {
                let mut adj_comments = 0;
                for (i, token) in ws.iter().rev().enumerate() {
                    match token.symbol {
                        COMMENT => {
                            adj_comments = i + 1;
                        }
                        WHITESPACE => {
                            let text = &self.text[token.range()];
                            if text.bytes().filter(|&b| b == b'\n').count() >= 2 {
                                break
                            }
                        }
                        c => unreachable!("not a ws: {:?}", c),
                    }
                }
                adj_comments
            }
            _ => 0,
        }
    }

    fn trailing_ws(&self, ws: &[lexer::Token], s: TomlSymbol) -> usize {
        match s {
            DOC => ws.len(),
            KEY_VAL => {
                let mut adj_comments = 0;
                for (i, token) in ws.iter().enumerate() {
                    match token.symbol {
                        COMMENT => {
                            adj_comments = i + 1;
                        }
                        WHITESPACE => {
                            let text = &self.text[token.range()];
                            if text.contains('\n') {
                                break
                            }
                        }
                        c => unreachable!("not a ws: {:?}", c),
                    }
                }
                adj_comments
            }
            _ => 0,
        }
    }

    fn whitespace(&self) -> &'t [lexer::Token] {
        let start = self.pos;
        let mut end = start;
        loop {
            match end.get(&self.tokens.raw_tokens) {
                Some(token) if !token.is_significant() => {
                    end += 1
                }
                _ => break,
            }
        }
        &self.tokens.raw_tokens[start.0 as usize..end.0 as usize]
    }

    fn bump(&mut self, s: Option<TomlSymbol>) {
        let t = self.tokens.raw_tokens[self.pos];
//        eprintln!("consumed {:?} at {:?}", t.symbol, self.pos);
        let s = s.unwrap_or(t.symbol).0;
        self.builder.leaf(s, t.len);
        self.pos += 1;
    }
}
