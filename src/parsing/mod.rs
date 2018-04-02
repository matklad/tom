use parse_tree::{Symbol, ParseTree, BottomUpBuilder};
use symbols;

mod grammar;

pub trait Events {
    fn shift(&mut self, symbol: Symbol, start: usize, end: usize);
    fn reduce(&mut self, symbol: Symbol, n_symbols: usize);
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

    fn finish(self, text: String) -> ParseTree {
        self.inner.finish(text)
    }

    fn shift_ws(&mut self, current: usize) {
        let len = current - self.prev;
        if len != 0 {
            self.stack.push(true);
            self.inner.shift(symbols::WHITESPACE, (len as u32).into());
        }
    }
}

impl Events for Builder {
    fn shift(&mut self, symbol: Symbol, start: usize, end: usize) {
        self.shift_ws(start);
        self.stack.push(false);
        self.prev = end;
        let len = end - start;
        self.inner.shift(symbol, (len as u32).into())
    }

    fn reduce(&mut self, symbol: Symbol, mut n_symbols: usize) {
        // trailing space
        if symbol == symbols::FILE {
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
        if symbol == symbols::FILE {
            while let Some(&is_ws) = self.stack.last() {
                if is_ws {
                    self.stack.pop().unwrap();
                    to_reduce += 1;
                }
            }
        }
        self.inner.reduce(symbol, to_reduce);
        self.stack.push(false);
    }
}

pub(crate) fn parse(text: String) -> ParseTree {
    symbols::register();
    let p = grammar::TomlFileParser::new();
    let mut builder = Builder::new(text.len());
    p.parse(&mut builder, &text).unwrap();
    builder.finish(text)
}
