//! FIXME: write short doc here

pub /*(crate)*/ trait ChunkedText where Self: Sized {
    fn for_each_chunk<F: FnMut(&str) -> Result<(), T>, T>(self, f: F) -> Result<(), T>;

    fn contains_char(self, c: char) -> bool {
        self.for_each_chunk(|chunk| {
            if chunk.contains(c) {
                return Err(());
            }
            Ok(())
        }).is_err()
    }

    fn write_to(self, buff: &mut String) {
        self.for_each_chunk(|chunk| {
            buff.push_str(chunk);
            Ok::<(), ()>(())
        }).unwrap();
    }

    fn into_string(self) -> String {
        let mut buff = String::new();
        self.write_to(&mut buff);
        buff
    }
}

impl<'a, I: Iterator<Item=&'a str>> ChunkedText for I {
    fn for_each_chunk<F: FnMut(&str) -> Result<(), T>, T>(mut self, f: F) -> Result<(), T> {
        self.try_for_each(f)
    }
}
