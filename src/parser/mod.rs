use parse_tree::{TopDownBuilder, ParseTree};
use {TomlSymbol, symbol::DOC};

mod lexer;
mod grammar;

pub(crate) fn parse(input: &str) -> ParseTree {
    let tokens = lexer::tokenize(input);
    let mut sink = EventSink::new(&tokens);
    {
        let mut parser = Parser {
            sink: &mut sink,
            tokens: &tokens,
            pos: 0,
        };
        parser.parse();
    }
    sink.builder.finish()
}

struct Parser<'s, 't: 's> {
    sink: &'s mut EventSink<'t>,
    tokens: &'t lexer::Tokens,
    pos: usize,
}

struct EventSink<'t> {
    pos: lexer::Pos,
    tokens: &'t lexer::Tokens,
    builder: TopDownBuilder,
}

impl<'t> EventSink<'t> {
    fn new(tokens: &'t lexer::Tokens) -> Self {
        EventSink {
            pos: lexer::Pos(0),
            tokens,
            builder: TopDownBuilder::new(),
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

    fn leading_ws(&self, ws: &[lexer::Token], s: TomlSymbol) -> usize {
        if s == DOC { ws.len() } else { 0 }
    }

    fn trailing_ws(&self, ws: &[lexer::Token], s: TomlSymbol) -> usize {
        if s == DOC { ws.len() } else { 0 }
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
        &self.tokens.raw_tokens[start.0 as usize .. end.0 as usize]
    }

    fn bump(&mut self, s: Option<TomlSymbol>) {
        let t = self.tokens.raw_tokens[self.pos];
//        eprintln!("consumed {:?} at {:?}", t.symbol, self.pos);
        let s = s.unwrap_or(t.symbol).0;
        self.builder.leaf(s, t.len);
        self.pos += 1;
    }
}

