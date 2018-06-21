use typed_arena::Arena;
use {TomlDoc, CstNode, ast};

pub struct Factory {
    arena: Arena<TomlDoc>,
}

impl Factory {
    pub fn new() -> Factory {
        Factory {
            arena: Arena::new(),
        }
    }

    pub fn key(&self, name: &str) -> ast::Key {
        self.entry_raw(format!("{} = 92", escaped_key(name)))
            .keys().next().unwrap()
    }

    pub fn value_string(&self, val: &str) -> ast::Value {
        //TODO: escaping
        self.entry_raw(format!("foo = {:?}", val)).value()
    }

    pub fn value_number(&self, val: i64) -> ast::Value {
        //TODO: escaping
        self.entry_raw(format!("foo = {}", val)).value()
    }

    pub fn value_bool(&self, val: bool) -> ast::Value {
        self.entry_raw(format!("foo = {}", val)).value()
    }

    pub fn value_dict<'a>(&self, entries: impl Iterator<Item=ast::Entry<'a>>) -> ast::Value {
        let buff = join(entries, '{', '}');
        self.entry_raw(format!("foo = {}", buff)).value()
    }

    pub fn value_array<'a>(&self, entries: impl Iterator<Item=ast::Value<'a>>) -> ast::Value {
        let buff = join(entries, '[', ']');
        self.entry_raw(format!("foo = {}", buff)).value()
    }

    pub fn entry<'a>(&self, keys: impl Iterator<Item=ast::Key<'a>>, value: ast::Value) -> ast::Entry {
        let mut buff = String::new();
        join_to(&mut buff, keys, ".", "", "");
        buff.push_str(" = ");
        buff.push_str(value.cst().text());
        self.entry_raw(buff)
    }

    pub fn table(&self) -> TableBuilder {
        TableBuilder::new(self)
    }

    fn doc(&self, text: String) -> &TomlDoc {
        self.arena.alloc(TomlDoc::new(text))
    }

    fn entry_raw(&self, text: String) -> ast::Entry {
        let doc = self.doc(text);
        doc.ast().entries().next().unwrap()
    }
}

pub fn join<'a, A: Into<CstNode<'a>>>(
    items: impl Iterator<Item=A>,
    left: char, right: char
) -> String {
    let mut buff = String::new();
    buff.push(left);
    join_to(&mut buff, items, ", ", " ", " ");
    buff.push(right);
    buff
}

pub fn join_to<'a, A: Into<CstNode<'a>>>(
    buff: &mut String,
    items: impl Iterator<Item=A>,
    sep: &str,
    before_first: &str, after_last: &str
) {
    let mut first = true;
    for item in items {
        if first {
            buff.push_str(before_first);
        }

        if !first {
            buff.push_str(sep);
        }
        first = false;
        buff.push_str(item.into().text());
    }
    if !first {
        buff.push_str(after_last);
    }
}

pub struct TableBuilder<'f, 'e> {
    factory: &'f Factory,
    keys: Vec<String>,
    entries: Vec<ast::Entry<'e>>,
}

impl<'f, 'e> TableBuilder<'f, 'e> {
    fn new(factory: &'f Factory) -> Self {
        TableBuilder {
            factory,
            keys: Vec::new(),
            entries: Vec::new(),
        }
    }

    pub fn with_name(mut self, key: &str) -> Self {
        if !self.keys.is_empty() {
            covered_by!("table_with_two_names");
            panic!("table header is already specified, can't reset to {:?}", key)
        }
        self.keys.push(key.to_owned());
        self
    }

    pub fn with_names<'a>(mut self, keys: impl Iterator<Item=&'a str>) -> Self {
        assert!(self.keys.is_empty());
        self.keys.extend(keys.map(str::to_owned));
        self
    }

    pub fn with_entries(mut self, entries: impl Iterator<Item=ast::Entry<'e>>) -> Self {
        self.entries.extend(entries);
        self
    }

    pub fn build(self) -> ast::Table<'f> {
        if self.keys.is_empty() {
            covered_by!("table_without_name");
            panic!("");
        }
        let mut buff = String::from("[");
        let mut first = true;
        for key in self.keys {
            if !first {
                buff.push_str(".")
            }
            first = false;
            buff.push_str(&escaped_key(&key));
        }
        buff.push_str("]");
        for e in self.entries {
            buff.push_str("\n");
            buff.push_str(e.cst().text());
        }
        let doc = self.factory.doc(buff);
        doc.ast().tables().next().unwrap()
    }
}

fn escaped_key(key: &str) -> String {
    if key.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            key.to_string()
        } else {
        format!("{:?}", key)
    }
}
