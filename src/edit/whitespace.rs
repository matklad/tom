use {
    TomlNode,
    symbol::*,
};

pub enum Location<'f> {
    Between(TomlNode<'f>, TomlNode<'f>),
    OnEdge {
        child: TomlNode<'f>,
        parent: TomlNode<'f>,
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

fn ws_between(left: TomlNode, right: TomlNode) -> String {
    match (left.symbol(), right.symbol()) {
        (KEY_VAL, KEY_VAL) | (TABLE_HEADER, KEY_VAL) => String::from("\n"),
        (TABLE, TABLE) | (KEY_VAL, TABLE) => String::from("\n\n"),
        _ => String::new(),
    }
}

fn ws_before(_: TomlNode, _: TomlNode) -> String {
    String::new()
}

fn ws_after(child: TomlNode, parent: TomlNode) -> String {
    if parent.symbol() == DOC && child.symbol() != WHITESPACE {
        String::from("\n")
    } else {
        String::new()
    }
}

