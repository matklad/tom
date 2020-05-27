mod grammar;
mod lexer;

use ::{
    symbol::*,
    SyntaxNode, GreenBuilder, Symbol, SmolStr, TextRange,
    SyntaxError,
};

pub(crate) fn parse(input: &str) -> SyntaxNode {
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
    let green = sink.builder.finish();
    SyntaxNode::new(green, sink.errors)
}

struct Parser<'s, 't: 's> {
    sink: &'s mut EventSink<'t>,
    tokens: &'t lexer::Tokens,
    pos: usize,
}

struct EventSink<'t> {
    pos: usize,
    text: &'t str,
    tokens: &'t lexer::Tokens,
    builder: GreenBuilder,
    errors: Vec<SyntaxError>,
}

impl<'t> EventSink<'t> {
    fn new(text: &'t str, tokens: &'t lexer::Tokens) -> Self {
        EventSink {
            pos: 0,
            text,
            tokens,
            builder: GreenBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn start(&mut self, s: Symbol) {
        let ws = self.whitespace();
        let n = self.leading_ws(ws, s);
        for _ in 0..(ws.len() - n) {
            self.bump(None)
        }
        self.builder.start_internal(s);
    }

    fn finish(&mut self, s: Symbol) {
        let ws = self.whitespace();
        let n = self.trailing_ws(ws, s);
        for _ in 0..n {
            self.bump(None)
        }
        self.builder.finish_internal();
    }

    fn token(&mut self, pos: usize, s: Option<Symbol>) {
        while self.pos < pos {
            self.bump(None)
        }
        self.bump(s);
    }

    fn error(&mut self, message: impl Into<String>) {
        if self.tokens.raw_tokens.is_empty() {
            self.errors.push(SyntaxError {
                range: TextRange::from_to(0.into(), 0.into()),
                message: message.into(),
            });
            return;
        }

        let mut pos = self.pos;
        if pos == self.tokens.raw_tokens.len() {
            pos -= 1;
        }
        let mut tok = &self.tokens.raw_tokens[pos];
        loop {
            match &self.tokens.raw_tokens.get(pos) {
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
            range: tok.range,
            message: message.into(),
        })
    }

    fn leading_ws(&self, ws: &[lexer::Token], s: Symbol) -> usize {
        match s {
            DOC => ws.len(),
            ENTRY | TABLE => {
                let mut adj_comments = 0;
                for (i, token) in ws.iter().rev().enumerate() {
                    match token.symbol {
                        COMMENT => {
                            adj_comments = i + 1;
                        }
                        WHITESPACE => {
                            let text = &self.text[token.range];
                            if text.bytes().filter(|&b| b == b'\n').count() >= 2 {
                                break;
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

    fn trailing_ws(&self, ws: &[lexer::Token], s: Symbol) -> usize {
        match s {
            DOC => ws.len(),
            ENTRY => {
                let mut adj_comments = 0;
                for (i, token) in ws.iter().enumerate() {
                    match token.symbol {
                        COMMENT => {
                            adj_comments = i + 1;
                        }
                        WHITESPACE => {
                            let text = &self.text[token.range];
                            if text.contains('\n') {
                                break;
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
            match &self.tokens.raw_tokens.get(end) {
                Some(token) if !token.is_significant() => end += 1,
                _ => break,
            }
        }
        &self.tokens.raw_tokens[start..end]
    }

    fn bump(&mut self, s: Option<Symbol>) {
        let t = self.tokens.raw_tokens[self.pos];
        let s = s.unwrap_or(t.symbol);
        let text: SmolStr = self.text[t.range].into();
        self.builder.leaf(s, text);
        self.pos += 1;
    }
}
