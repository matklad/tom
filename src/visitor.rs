use {
    CstNode,
    ast,
};

pub(crate) struct Visitor<'f, 'c, C> {
    ctx: C,
    callbacks: Vec<Box<FnMut(&mut C, CstNode<'f>) + 'c>>,
}

pub(crate) fn visitor<'f, 'c, C>(ctx: C) -> Visitor<'f, 'c, C> {
    Visitor { ctx, callbacks: Vec::new() }
}

pub(crate) fn process<'f, 'c, C>(
    node: CstNode<'f>,
    mut v: Visitor<'f, 'c, C>,
) -> C {
    go(node, &mut v);
    return v.ctx;

    fn go<'f, 'c, C>(node: CstNode<'f>, v: &mut Visitor<'f, 'c, C>) {
        v.do_visit(node);
        for child in node.children() {
            go(child, v);
        }
    }
}

impl<'f, 'c, C> Visitor<'f, 'c, C> {
    pub fn visit<A: ast::AstNode<'f>, F: FnMut(&mut C, A) + 'c>(
        mut self,
        mut f: F,
    ) -> Self {
        let cb: Box<FnMut(&mut C, CstNode<'f>) + 'c> =
            Box::new(move |c, node| {
                match A::cast(node) {
                    None => (),
                    Some(a) => f(c, a),
                }
            });
        self.callbacks.push(cb);
        self
    }

    fn do_visit(&mut self, node: CstNode<'f>) {
        for cb in self.callbacks.iter_mut() {
            cb(&mut self.ctx, node);
        }
    }
}
