use string_interner::StringInterner;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct InternId(u32);

impl From<usize> for InternId {
    fn from(s: usize) -> InternId {
        InternId(s as u32)
    }
}

impl From<InternId> for usize {
    fn from(id: InternId) -> usize {
        id.0 as usize
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
