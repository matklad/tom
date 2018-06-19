use super::Parser;
use symbol::*;


struct Mark(TomlSymbol);

impl Drop for Mark {
    fn drop(&mut self) {
        if !::std::thread::panicking() {
            panic!("Mark dropped")
        }
    }
}

impl<'t, 's> Parser<'t, 's> {
    fn start(&mut self, s: TomlSymbol) -> Mark {
        self.sink.start(s);
        Mark(s)
    }

    fn finish(&mut self, m: Mark) {
        self.sink.finish(m.0);
        ::std::mem::forget(m)
    }

    fn current(&self) -> TomlSymbol {
        if self.pos == self.tokens.significant.len() {
            return EOF;
        }
        let pos = self.tokens.significant[self.pos];
        self.tokens.raw_tokens[pos].symbol
    }

    fn eat(&mut self, s: TomlSymbol) {
        if self.current() == s {
            self.bump()
        } else {
            self.bump_error()
        }
    }

    fn bump(&mut self) {
        if self.pos == self.tokens.significant.len() {
            panic!("bumping past EOF");
        }
        let pos = self.tokens.significant[self.pos];
        self.sink.token(pos, None);
        self.pos += 1;
    }

    fn bump_remap(&mut self, s: TomlSymbol) {
        if self.pos == self.tokens.significant.len() {
            panic!("bumping past EOF");
        }
        let pos = self.tokens.significant[self.pos];
        self.sink.token(pos, Some(s));
        self.pos += 1;
    }

    fn bump_error(&mut self) {
        let m = self.start(ERROR);
        self.bump();
        self.finish(m);
    }
}

impl<'s, 't> Parser<'s, 't> {
    pub(crate) fn parse(&mut self) {
        self.doc();
    }

    fn doc(&mut self) {
        let m = self.start(DOC);
        while self.current() != EOF {
            match self.current() {
                // test
                // foo = 92
                | BARE_KEY | BARE_KEY_OR_NUMBER | BARE_KEY_OR_DATE
                | BASIC_STRING | LITERAL_STRING
                => self.key_val(),
                _ => self.bump_error(),
            }
        }
        self.finish(m);
    }

    fn key_val(&mut self) {
        let m = self.start(KEY_VAL);
        self.key();
        self.eat(EQ);
        self.val();
        self.finish(m);
    }

    fn key(&mut self) {
        match self.current() {
            // test
            // foo = 92
            // 92 = 92
            // 1914-08-26 = 92
            BARE_KEY | BARE_KEY_OR_NUMBER | BARE_KEY_OR_DATE =>
                self.bump_remap(BARE_KEY),
            // test
            // "foo" = 92
            // 'bar' = 92
            BASIC_STRING | LITERAL_STRING =>
                self.bump(),
            _ => unreachable!()
        }
    }

    fn val(&mut self) {
        match self.current() {
            // test
            // a = 92
            // b = 8.5
            BARE_KEY_OR_NUMBER | NUMBER =>
                self.bump_remap(NUMBER),
            // test
            // a = true
            // b = false
            BOOL => self.bump(),
            // a = 1914-08-26
            // b = 1979-05-27T07:32:00-08:00
            BARE_KEY_OR_DATE | DATE_TIME =>
                self.bump_remap(DATE_TIME),
            // test
            // f = "hello\nworld"
            // g = """
            //   hello
            //   world
            // """
            BASIC_STRING | MULTILINE_BASIC_STRING => self.bump(),
            // test
            // f = 'hello\nworld'
            // g = '''
            //   hello
            //   world
            // '''
            LITERAL_STRING | MULTILINE_LITERAL_STRING => self.bump(),
            L_BRACK => unimplemented!("ARRAY"),
            L_CURLY => unimplemented!("DICT"),
            c => unreachable!("{:?} not in FIRST(val) ", c),
        }
    }
}
