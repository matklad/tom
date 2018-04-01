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
}

impl Builder {
    fn new() -> Builder {
        Builder {
            inner: BottomUpBuilder::new(),
            stack: Vec::new(),
            prev: 0,
        }
    }

    fn finish(self, text: String) -> ParseTree {
        self.inner.finish(text)
    }
}

impl Events for Builder {
    fn shift(&mut self, symbol: Symbol, start: usize, end: usize) {
        let gap = start - self.prev;
        if gap != 0 {
            self.stack.push(true);
            self.inner.shift(symbols::WHITESPACE, (gap as u32).into());
        }
        self.stack.push(false);
        self.prev = end;
        let len = end - start;
        self.inner.shift(symbol, (len as u32).into())
    }

    fn reduce(&mut self, symbol: Symbol, mut n_symbols: usize) {
        let mut to_reduce = 0;
        while n_symbols > 0 {
            let is_ws = self.stack.pop().unwrap();
            to_reduce += 1;
            if !is_ws {
                n_symbols -= 1;
            }
        }
        self.inner.reduce(symbol, to_reduce);
        self.stack.push(false);
    }
}

pub(crate) fn parse(text: String) -> ParseTree {
    symbols::register();
    let p = grammar::TomlFileParser::new();
    let mut builder = Builder::new();
    p.parse(&mut builder, &text).unwrap();
    builder.finish(text)
}
