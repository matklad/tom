use std::num::NonZeroU32;
use {TextRange, TextUnit};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct InternId(NonZeroU32);

impl InternId {
    fn from_idx(idx: usize) -> InternId {
        InternId(NonZeroU32::new((idx + 1) as u32).unwrap())
    }
    fn to_idx(self) -> usize {
        (self.0.get() as usize) - 1
    }
}

pub(crate) struct Intern {
    data: String,
    interned: Vec<TextRange>,
    sorted: Vec<InternId>,
}

impl Intern {
    pub fn new() -> Intern {
        Intern {
            data: String::new(),
            interned: Vec::new(),
            sorted: Vec::new(),
        }
    }

    pub fn resolve(&self, id: InternId) -> &str {
        let range = self.interned[id.to_idx()];
        &self.data[range]
    }

    pub fn intern(&mut self, val: &str) -> InternId {
        let idx = self
            .sorted
            .binary_search_by(|&id| self.resolve(id).cmp(val));

        match idx {
            Ok(idx) => self.sorted[idx],
            Err(insertion_point) => {
                let range = TextRange::offset_len(
                    TextUnit::of_str(&self.data),
                    TextUnit::of_str(val),
                );
                let id = InternId::from_idx(self.interned.len());

                self.data.push_str(val);
                self.interned.push(range);
                self.sorted.insert(insertion_point, id);

                id
            }
        }
    }
}
