use {
    CstNode,
    symbol::*,
};

pub enum Location<'f> {
    Between(CstNode<'f>, CstNode<'f>),
    OnEdge {
        child: CstNode<'f>,
        parent: CstNode<'f>,
        edge: Edge,
    },
}

#[derive(Copy, Clone)]
pub enum Edge { Left, Right }

pub fn compute_ws(loc: Location) -> String {
    match loc {
        Location::Between(left, right) =>
            ws_between(left, right),
        Location::OnEdge { child, parent, edge } =>
            match edge {
                Edge::Left => ws_before(child, parent),
                Edge::Right => ws_after(child, parent),
            }
    }
}

fn ws_between(left: CstNode, right: CstNode) -> String {
    match (left.symbol(), right.symbol()) {
        (ENTRY, ENTRY) | (TABLE_HEADER, ENTRY) => String::from("\n"),
        (TABLE, TABLE) | (ENTRY, TABLE) => String::from("\n\n"),
        _ => String::new(),
    }
}

fn ws_before(_: CstNode, _: CstNode) -> String {
    String::new()
}

fn ws_after(child: CstNode, parent: CstNode) -> String {
    if parent.symbol() == DOC && child.symbol() != WHITESPACE {
        String::from("\n")
    } else {
        String::new()
    }
}

