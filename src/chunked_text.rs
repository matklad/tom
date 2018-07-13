pub /*(crate)*/ trait ChunkedText {
    fn for_each_chunk<F: FnMut(&str) -> Result<(), T>, T>(&self, f: F) -> Result<(), T>;

    fn contains_char(&self, c: char) -> bool {
        self.for_each_chunk(|chunk| {
            if chunk.contains(c) {
                return Err(());
            }
            Ok(())
        }).is_err()
    }

    fn write_to(&self, buff: &mut String) {
        self.for_each_chunk(|chunk| {
            buff.push_str(chunk);
            Ok::<(), ()>(())
        }).unwrap();
    }

    fn to_string(&self) -> String {
        let mut buff = String::new();
        self.write_to(&mut buff);
        buff
    }
}
