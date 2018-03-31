use parse_tree::Symbol;

pub trait Events {
    fn shift(&mut self, symbol: Symbol, start: usize, end: usize);
    fn reduce(&mut self, symbol: Symbol, n_symbols: usize);
}
