use TomlFile;
use ast::{self, KeyValueOwner, AstNode};
use typed_arena::Arena;

pub struct Factory {
    arena: Arena<TomlFile>
}

impl Factory {
    pub fn new() -> Factory {
        Factory { arena: Arena::new() }
    }

    pub fn val_string(&self, val: &str) -> ast::Val {
        let file = self.file(format!("foo = {:?}", val));
        file.ast().entries().next().unwrap().val()
    }

    pub fn key_val(&self, key: &str, val: ast::Val) -> ast::KeyVal {
        let file = self.file(format!("{} = {}", escaped_key(key), val.node().text()));
        file.ast().entries().next().unwrap()
    }

    pub fn table(
        &self,
        keys: &mut Iterator<Item=&str>,
        entries: &mut Iterator<Item=ast::KeyVal>,
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
    if key.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        key.to_string()
    } else {
        format!("'{}'", key)
    }
}


#[test]
fn test_create_key_val() {
    let f = Factory::new();
    let val = f.val_string("1.0");
    let kv = f.key_val("foo", val);
    assert_eq!(kv.node().text(), r#"foo = "1.0""#);
}

#[test]
fn test_create_table() {
    let f = Factory::new();
    let va = f.val_string("1.0");
    let a = f.key_val("foo", va);
    let vb = f.val_string("0.0.1");
    let b = f.key_val("bar", vb);
    let table = f.table(
        &mut vec!["target", "x86_64.json", "dependencies"].into_iter(),
        &mut vec![a, b].into_iter()
    );
    assert_eq!(table.node().text(), r#"
[target.'x86_64.json'.dependencies]
foo = "1.0"
bar = "0.0.1"
"#.trim());
}
