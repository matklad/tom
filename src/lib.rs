#[macro_use]
extern crate parse_tree;

use parse_tree::{BottomUpBuilder};

use parse_tree::Symbol;
use parse_tree::ParseTree;

pub mod grammar;
mod symbols;
mod events;

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

    fn finish(self) -> ParseTree {
        self.inner.finish()
    }
}

impl events::Events for Builder {
    fn shift(&mut self, symbol: Symbol, start: usize, end: usize) {
        let gap = start - self.prev;
        let has_ws = if gap != 0 {
            self.inner.shift(symbols::WHITESPACE, (gap as u32).into());
            true
        } else {
            false
        };
        self.stack.push(has_ws);
        self.prev = end;
        let len = end - start;
        self.inner.shift(symbol, (len as u32).into())
    }

    fn reduce(&mut self, symbol: Symbol, n_symbols: usize) {
        let n = self.stack.iter().rev().take(n_symbols)
            .map(|&has_ws| if has_ws { 2 } else { 1 })
            .sum();
        self.inner.reduce(symbol, n);
        for _ in 0..n_symbols {
            self.stack.pop().unwrap();
        }
        self.stack.push(false);
    }
}

pub fn parse(text: &str) -> ParseTree {
    symbols::register();
    let p = grammar::TomlFileParser::new();
    let mut builder = Builder::new();
    p.parse(&mut builder, text).unwrap();
    builder.finish()
}
