use ::{
    TomlFile, TomlNode,
    symbol::*,
};

#[derive(Debug)]
pub struct Edit<'f> {
    file: &'f TomlFile,
    ops: Vec<(TomlNode<'f>, Op<'f>)>,
}


#[derive(Debug)]
enum Op<'f> {
    Delete,
    Replace(TomlNode<'f>),
    Rewrite(String),
    Insert {
        children: Vec<TomlNode<'f>>,
        position: usize,
    },
}

impl<'f> Edit<'f> {
    pub fn new(file: &'f TomlFile) -> Edit {
        Edit {
            file,
            ops: Vec::new(),
        }
    }

    pub fn replace(&mut self, node: TomlNode<'f>, replacement: TomlNode<'f>) {
        self.op(node, Op::Replace(replacement));
    }

    pub fn replace_with_text(&mut self, node: TomlNode<'f>, replacement: String) {
        self.op(node, Op::Rewrite(replacement));
    }

    pub fn append_child(&mut self, parent: TomlNode<'f>, child: TomlNode<'f>) {
        self.append_children(parent, vec![child]);
    }

    pub fn append_children(&mut self, parent: TomlNode<'f>, children: Vec<TomlNode<'f>>) {
        self.op(parent, Op::Insert {
            children,
            position: parent.children().count(),
        });
    }

    pub fn delete(&mut self, node: TomlNode<'f>) {
        self.op(node, Op::Delete);
    }

    pub fn finish(self) -> String {
        let root = self.file.parse_tree();
        let mut res = self.rendered(root);
        if !res.ends_with("\n") {
            res += "\n";
        }
        res
    }

    fn op(&mut self, target: TomlNode<'f>, op: Op<'f>) {
        self.ops.push((target, op))
    }
}

impl<'f> Edit<'f> {
    fn rendered(&self, node: TomlNode<'f>) -> String {
        for op in self.ops_for(node) {
            return match op {
                Op::Delete => String::new(),
                Op::Rewrite(text) => text.to_owned(),
                Op::Replace(replacement) => self.rendered(*replacement),
                Op::Insert { children, position } => {
                    let mut buff = String::new();
                    let mut prev_child = None;
                    for child in node.children().take(*position) {
                        buff += &self.rendered(child);
                        prev_child = Some(child);
                    }
                    for &child in children {
                        if let Some(prev) = prev_child {
                            buff += &compute_ws(prev, child);
                        }
                        buff += &self.rendered(child);
                        prev_child = Some(child);
                    }
                    for child in node.children().skip(*position) {
                        if let Some(prev) = prev_child {
                            buff += &compute_ws(prev, child);
                            prev_child = None;
                        }
                        buff += &self.rendered(child);
                    }
                    buff
                }
            };
        }

        if node.is_leaf() {
            node.text().to_owned()
        } else {
            let mut buff = String::new();
            for child in node.children() {
                buff += &self.rendered(child);
            }
            buff
        }
    }

    fn ops_for(&self, target: TomlNode<'f>) -> impl Iterator<Item=&Op<'f>> {
        self.ops.iter()
            .filter(move |(node, _)| *node == target)
            .map(|(_, op)| op)
    }
}

fn compute_ws(left: TomlNode, right: TomlNode) -> String {
    match (left.symbol(), right.symbol()) {
        (KEY_VAL, KEY_VAL) |
        (TABLE_HEADER, KEY_VAL) => String::from("\n"),
        (TABLE, TABLE) |
        (KEY_VAL, TABLE) => String::from("\n\n"),
        _ => String::new()
    }
}
