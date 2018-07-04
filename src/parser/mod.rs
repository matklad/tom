mod grammar;
mod lexer;

use {
    intern::Intern,
    symbol::{COMMENT, DOC, ENTRY, TABLE, WHITESPACE},
    tree::{NodeId, TreeData, InsertPos},
    Symbol, SyntaxError, Tree,
};

pub(crate) struct ParseTree {
    pub tree: Tree,
    pub intern: Intern,
    pub errors: Vec<SyntaxError>,
}

pub(crate) fn parse(input: &str, parse_tree: &mut ParseTree, root: NodeId) {
    let tokens = lexer::tokenize(input);
    let mut sink = EventSink::new(input, &tokens, parse_tree, root);
    let mut parser = Parser {
        sink: &mut sink,
        tokens: &tokens,
        pos: 0,
    };
    parser.parse();
}

struct Parser<'s, 't: 's, 'a: 's> {
    sink: &'s mut EventSink<'t, 'a>,
    tokens: &'t lexer::Tokens,
    pos: usize,
}

struct EventSink<'t, 'a> {
    pos: lexer::Pos,
    text: &'t str,
    tokens: &'t lexer::Tokens,
    parse_tree: &'a mut ParseTree,
    stack: Vec<NodeId>,
}

impl<'t, 'a> EventSink<'t, 'a> {
    fn new(text: &'t str, tokens: &'t lexer::Tokens, parse_tree: &'a mut ParseTree, root: NodeId) -> Self {
        let stack = vec![root];

        EventSink {
            pos: lexer::Pos(0),
            text,
            tokens,
            parse_tree,
            stack,
        }
    }

    fn start(&mut self, s: Symbol) {
        // eprintln!("start {:?} at {:?}", s, self.pos);
        let ws = self.whitespace();
        let n = self.leading_ws(ws, s);
        for _ in 0..(ws.len() - n) {
            self.bump(None)
        }
        if s != DOC {
            let node = self.parse_tree.tree.new_internal(s);
            let top = self.top();
            self.parse_tree.tree.insert_child(top, node, InsertPos::Last);

            self.stack.push(node);
        }
    }

    fn finish(&mut self, s: Symbol) {
        // eprintln!("finished {:?} at {:?}", s, self.pos);
        let ws = self.whitespace();
        let n = self.trailing_ws(ws, s);
        for _ in 0..n {
            self.bump(None)
        }
        let node = self.stack.pop().unwrap();
        match node.data(&self.parse_tree.tree) {
            TreeData::Internal(&sym) => assert_eq!(sym, s),
            _ => (),
        }
    }

    fn token(&mut self, pos: lexer::Pos, s: Option<Symbol>) {
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

        self.parse_tree.errors.push(SyntaxError {
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
            match end.get(&self.tokens.raw_tokens) {
                Some(token) if !token.is_significant() => end += 1,
                _ => break,
            }
        }
        &self.tokens.raw_tokens[start.0 as usize..end.0 as usize]
    }

    fn bump(&mut self, s: Option<Symbol>) {
        let t = self.tokens.raw_tokens[self.pos];
        //        eprintln!("consumed {:?} at {:?}", t.symbol, self.pos);
        let s = s.unwrap_or(t.symbol);
        let text = &self.text[t.range];
        let intern_id = self.parse_tree.intern.intern(text);
        let leaf = self.parse_tree.tree.new_leaf((s, intern_id));
        let top = self.top();
        self.parse_tree.tree.insert_child(top, leaf, InsertPos::Last);
        self.pos += 1;
    }

    fn top(&self) -> NodeId {
        *self.stack.last().unwrap()
    }
}
