use {TomlSymbol, WHITESPACE, DOC, ERROR};
use parse_tree::{ParseTree, BottomUpBuilder};

mod grammar;
mod rd;

pub trait Events {
    fn shift(&mut self, symbol: TomlSymbol, start: usize, end: usize);
    fn reduce(&mut self, symbol: TomlSymbol, n_symbols: usize);
}

struct Builder {
    inner: BottomUpBuilder,
    stack: Vec<bool>,
    prev: usize,
    total: usize,
}

impl Builder {
    fn new(total: usize) -> Builder {
        Builder {
            inner: BottomUpBuilder::new(),
            stack: Vec::new(),
            prev: 0,
            total,
        }
    }

    fn finish(self) -> ParseTree {
        self.inner.finish()
    }

    fn shift_ws(&mut self, current: usize) {
        let len = current - self.prev;
        if len != 0 {
            self.stack.push(true);
            self.inner.shift(WHITESPACE.0, (len as u32).into());
        }
    }
}

impl Events for Builder {
    fn shift(&mut self, symbol: TomlSymbol, start: usize, end: usize) {
        self.shift_ws(start);
        self.stack.push(false);
        self.prev = end;
        let len = end - start;
        self.inner.shift(symbol.0, (len as u32).into())
    }

    fn reduce(&mut self, symbol: TomlSymbol, mut n_symbols: usize) {
        // trailing space
        if symbol == DOC {
            let total = self.total;
            self.shift_ws(total);
        }
        let mut to_reduce = 0;
        while n_symbols > 0 {
            let is_ws = self.stack.pop().unwrap();
            to_reduce += 1;
            if !is_ws {
                n_symbols -= 1;
            }
        }
        // leading space
        if symbol == DOC {
            while let Some(&is_ws) = self.stack.last() {
                if is_ws {
                    self.stack.pop().unwrap();
                    to_reduce += 1;
                }
            }
        }
        self.inner.reduce(symbol.0, to_reduce);
        self.stack.push(false);
    }
}

pub(crate) fn parse(text: &str) -> ParseTree {
    let p = grammar::DocParser::new();
    let mut builder = Builder::new(text.len());
    if p.parse(&mut builder, text).is_ok() {
        builder.finish()
    } else {
        let mut builder = Builder::new(text.len());
        builder.shift(ERROR, 0, text.len());
        builder.reduce(DOC, 1);
        builder.finish()
    }
}
