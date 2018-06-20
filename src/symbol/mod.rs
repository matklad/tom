use cst::Symbol;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TomlSymbol(pub(crate) Symbol);

pub(crate) struct SymbolInfo(u16, &'static str);

impl TomlSymbol {
    pub fn name(&self) -> &'static str {
        self.info().1
    }

    pub(crate) fn info(&self) -> &SymbolInfo {
        let idx = self.0;
        let idx = idx.0 as usize;
        &generated::SYMBOLS[idx]
    }
}

impl fmt::Debug for TomlSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "`{}", self.name())
    }
}

mod generated;
pub use self::generated::*;
