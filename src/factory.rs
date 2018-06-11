use TomlFile;
use ast::{self, AstNode, KeyValueOwner};
use typed_arena::Arena;

pub struct Factory {
    arena: Arena<TomlFile>,
}

impl Factory {
    pub fn new() -> Factory {
        Factory {
            arena: Arena::new(),
        }
    }

    pub fn val_string(&self, val: &str) -> ast::Val {
        //TODO: escaping
        let file = self.file(format!("foo = {:?}", val));
        file.ast().entries().next().unwrap().val()
    }

    pub fn val_bool(&self, val: bool) -> ast::Val {
        let file = self.file(format!("foo = {}", val));
        file.ast().entries().next().unwrap().val()
    }

    pub fn val_dict(&self, entries: &mut Iterator<Item = ast::KeyVal>) -> ast::Val {
        let mut buff = String::from("{");
        let mut first = true;
        for e in entries {
            buff.push_str(if first { " " } else { ", " });
            first = false;
            buff.push_str(e.node().text());
        }
        buff.push_str(" }");
        let file = self.file(format!("foo = {}", buff));
        file.ast().entries().next().unwrap().val()
    }

    pub fn key_val(&self, key: &str, val: ast::Val) -> ast::KeyVal {
        let file = self.file(format!("{} = {}", escaped_key(key), val.node().text()));
        file.ast().entries().next().unwrap()
    }

    pub fn table(
        &self,
        keys: &mut Iterator<Item = &str>,
        entries: &mut Iterator<Item = ast::KeyVal>,
    ) -> ast::Table {
        let mut buff = String::from("[");
        let mut first = true;
        for key in keys {
            if !first {
                buff.push_str(".")
            }
            first = false;
            buff.push_str(&escaped_key(key));
        }
        buff.push_str("]");
        for e in entries {
            buff.push_str("\n");
            buff.push_str(e.node().text());
        }
        let file = self.file(buff);
        file.ast().tables().next().unwrap()
    }

    fn file(&self, text: String) -> &TomlFile {
        self.arena.alloc(TomlFile::new(text))
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
