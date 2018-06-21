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

    pub fn doc(&self, text: String) -> ast::Doc {
        self.arena.alloc(TomlDoc::new(text))
            .ast()
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

    pub fn table<'a, 'b>(
        &self,
        keys: impl Iterator<Item=ast::Key<'a>>,
        entries: impl Iterator<Item=ast::Entry<'b>>
    ) -> ast::Table {
        self.table_impl(keys, entries, "[", "]")
    }

    pub fn array_table<'a, 'b>(
        &self,
        keys: impl Iterator<Item=ast::Key<'a>>,
        entries: impl Iterator<Item=ast::Entry<'b>>
    ) -> ast::Table {
        self.table_impl(keys, entries, "[[", "]]")
    }

    fn table_impl<'a, 'b>(
        &self,
        keys: impl Iterator<Item=ast::Key<'a>>,
        entries: impl Iterator<Item=ast::Entry<'b>>,
        left: &str, right: &str,
    ) -> ast::Table {
        let mut buff = String::new();
        buff.push_str(left);
        join_to(&mut buff, keys, ".", "", "");
        buff.push_str(right);
        for entry in entries {
            buff.push('\n');
            buff.push_str(entry.cst().text());
        }
        self.doc(buff).tables().next().unwrap()
    }

    fn entry_raw(&self, text: String) -> ast::Entry {
        self.doc(text).entries().next().unwrap()
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

fn escaped_key(key: &str) -> String {
    if key.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            key.to_string()
        } else {
        format!("{:?}", key)
    }
}
