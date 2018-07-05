use string_interner::StringInterner;
use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct InternId(NonZeroU32);

impl From<usize> for InternId {
    fn from(s: usize) -> InternId {
        InternId(NonZeroU32::new((s + 1) as u32).unwrap())
    }
}

impl From<InternId> for usize {
    fn from(id: InternId) -> usize {
        (id.0.get() as usize) - 1
    }
}

pub(crate) struct Intern {
    inner: StringInterner<InternId>,
}

impl Intern {
    pub fn new() -> Intern {
        Intern {
            inner: StringInterner::new(),
        }
    }

    pub fn intern(&mut self, val: &str) -> InternId {
        self.inner.get_or_intern(val)
    }

    pub fn resolve(&self, id: InternId) -> &str {
        self.inner.resolve(id).unwrap()
    }
}
