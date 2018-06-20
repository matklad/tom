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

    fn at(&self, lookahead: usize) -> TomlSymbol {
        let pos = self.pos + lookahead;
        if pos >= self.tokens.significant.len() {
            return EOF;
        }
        let pos = self.tokens.significant[pos];
        self.tokens.raw_tokens[pos].symbol
    }

    fn current(&self) -> TomlSymbol {
        self.at(0)
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
        if self.current() != EOF {
            let m = self.start(ERROR);
            self.bump();
            self.finish(m);
        }
    }
}

impl<'s, 't> Parser<'s, 't> {
    pub(crate) fn parse(&mut self) {
        self.doc();
    }

    // test
    // key = "value"
    //
    // [table]
    // a = 2
    //
    // [[array-table]]
    // b = 3
    fn doc(&mut self) {
        let m = self.start(DOC);
        self.entries();
        while self.current() != EOF {
            match self.current() {
                L_BRACK => {
                    if self.at(1) == L_BRACK {
                        self.array_table()
                    } else {
                        self.table()
                    }
                }
                _ => self.bump_error(),
            }
        }

        self.finish(m);
    }

    // test
    // foo = 92
    // 'bar' = 14
    fn entries(&mut self) {
        while self.current() != EOF && self.current() != L_BRACK {
            match self.current() {
                | BARE_KEY | BARE_KEY_OR_NUMBER | BARE_KEY_OR_DATE
                | BASIC_STRING | LITERAL_STRING =>
                    self.key_val(),
                _ => self.bump_error(),
            }
        }
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
            _ => self.bump_error(),
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
            // a = "hello\nworld"
            // b = """
            //   hello
            //   world
            // """
            BASIC_STRING | MULTILINE_BASIC_STRING => self.bump(),
            // test
            // a = 'hello\nworld'
            // b = '''
            //   hello
            //   world
            // '''
            LITERAL_STRING | MULTILINE_LITERAL_STRING => self.bump(),
            // test
            // a = [1, "foo"]
            L_BRACK => self.array(),
            // test
            // a = { "foo" = 1, bar = 2, }
            L_CURLY => self.dict(),
            // test
            // foo = _
            _ => self.bump_error(),
        }
    }

    fn array(&mut self) {
        assert_eq!(self.current(), L_BRACK);
        let m = self.start(ARRAY);
        self.bump();
        while self.current() != EOF && self.current() != R_BRACK {
            self.val();
            // test
            // a = []
            // b = [1]
            // c = [1,]
            // d = [,]
            if self.current() != R_BRACK {
                self.eat(COMMA)
            }
        }
        self.eat(R_BRACK);

        self.finish(m);
    }

    fn dict(&mut self) {
        assert_eq!(self.current(), L_CURLY);
        let m = self.start(DICT);
        self.bump();
        while self.current() != EOF && self.current() != R_CURLY {
            self.key_val();
            // test
            // a = {}
            // b = {foo=1}
            // c = {foo=1,}
            // d = {,}
            if self.current() != R_CURLY {
                self.eat(COMMA)
            }
        }
        self.eat(R_CURLY);

        self.finish(m);
    }

    // test
    // [table]
    // a = 1
    // b = 2
    fn table(&mut self) {
        assert_eq!(self.current(), L_BRACK);
        let m = self.start(TABLE);
        self.table_header(false);
        self.entries();
        self.finish(m)
    }

    // test
    // [[array-table]]
    // a = 1
    // b = 2
    fn array_table(&mut self) {
        assert!(self.at(0) == L_BRACK && self.at(1) == L_BRACK);
        let m = self.start(ARRAY_TABLE);
        self.table_header(true);
        self.entries();
        self.finish(m)
    }

    // test
    // [[table . 'header']]
    fn table_header(&mut self, array: bool) {
        assert_eq!(self.current(), L_BRACK);
        let m = self.start(TABLE_HEADER);
        self.bump();
        if array {
            assert_eq!(self.current(), L_BRACK);
            self.bump();
        }

        // test
        // []
        // [foo]
        // [foo.bar]
        // [foo.]
        let mut first = true;
        while self.current() != EOF && self.current() != R_BRACK {
            if !first {
                self.eat(DOT);
            }
            first = false;
            self.key();
        }

        self.eat(R_BRACK);
        if array {
            self.eat(R_BRACK);
        }
        self.finish(m);
    }
}
