use {AstNode, CstNode, TomlDoc};

pub(crate) struct Visitor<'c, C> {
    ctx: C,
    callbacks: Vec<Box<FnMut(&mut C, CstNode, &TomlDoc) + 'c>>,
}

pub(crate) fn visitor<'c, C>(ctx: C) -> Visitor<'c, C> {
    Visitor {
        ctx,
        callbacks: Vec::new(),
    }
}

pub(crate) fn process<'c, C>(node: CstNode, doc: &TomlDoc, mut v: Visitor<'c, C>) -> C {
    go(node, doc, &mut v);
    return v.ctx;

    fn go<'c, C>(node: CstNode, doc: &TomlDoc, v: &mut Visitor<'c, C>) {
        v.do_visit(node, doc);
        for child in node.children(doc) {
            go(child, doc, v);
        }
    }
}

impl<'c, C> Visitor<'c, C> {
    pub fn visit<A: AstNode, F: FnMut(&mut C, A) + 'c>(mut self, mut f: F) -> Self {
        let cb: Box<FnMut(&mut C, CstNode, &TomlDoc) + 'c> =
            Box::new(move |c, node, doc| match A::cast(node, doc) {
                None => (),
                Some(a) => f(c, a),
            });
        self.callbacks.push(cb);
        self
    }

    fn do_visit(&mut self, node: CstNode, doc: &TomlDoc) {
        for cb in self.callbacks.iter_mut() {
            cb(&mut self.ctx, node, doc);
        }
    }
}
