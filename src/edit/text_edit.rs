use parse_tree::{TextRange, TextUnit};

use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct TextEdit {
    pub ops: Vec<TextEditOp>,
}

#[derive(Clone, Debug)]
pub enum TextEditOp {
    Copy(TextRange),
    Insert(String),
}


impl TextEdit {
    pub fn apply(&self, text: &str) -> String {
        let mut result = String::new();
        for s in self.ops.iter() {
            match *s {
                TextEditOp::Copy(range) => result += &text[range],
                TextEditOp::Insert(ref s) => result += s,
            }
        }

        result.into()
    }
}

pub struct TextEditBuilder {
    segments: Vec<TextEditOp>,
    last_offset: TextUnit,
    text_len: TextUnit,

}

impl TextEditBuilder {
    pub fn new(text: &str) -> TextEditBuilder {
        TextEditBuilder {
            segments: Vec::new(),
            last_offset: 0.into(),
            text_len: (text.len() as u32).into()
        }
    }

    pub fn build(mut self) -> TextEdit {
        let len = self.text_len;
        self.advance_to(len);
        TextEdit { ops: self.segments }
    }

    pub fn insert(&mut self, offset: TextUnit, text: String) {
        self.advance_to(offset);
        self.insert_(text);
    }

    pub fn delete(&mut self, range: TextRange) {
        self.advance_to(range.start());
        self.delete_len(range.len());
    }

    pub fn replace(&mut self, range: TextRange, text: String) {
        self.advance_to(range.start());
        self.insert_(text.into());
        self.delete_len(range.len());
    }

    fn advance_to(&mut self, offset: TextUnit) {
        match self.last_offset.cmp(&offset) {
            Ordering::Less => self.copy_up_to(offset),
            Ordering::Equal => (),
            Ordering::Greater => panic!("Invalid edit"),
        }
    }

    fn copy_up_to(&mut self, offset: TextUnit) {
        let len = offset - self.last_offset;
        self.copy_len(len)
    }

    fn copy_len(&mut self, len: TextUnit) {
        let range = TextRange::from_len(self.last_offset, len);
        self.segments.push(TextEditOp::Copy(range));
        self.last_offset += len
    }

    fn insert_(&mut self, text: String) {
        self.segments.push(TextEditOp::Insert(text))
    }

    fn delete_len(&mut self, len: TextUnit) {
        self.last_offset += len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edits() {
        let text: String = "Hello, World!".into();
        let edit = {
            let mut e = TextEditBuilder::new(&text);
            e.replace(TextRange::from_len(0.into(), 5.into()), "Goodbye".to_string());
            e.insert(7.into(), "cruel ".to_string());
            e.delete(TextRange::from_len(12.into(), 1.into()));
            e.build()
        };
        let new_text = edit.apply(&text);
        assert_eq!(new_text, "Goodbye, cruel World");
    }
}
