use {TomlDoc, CstNode};

#[derive(Debug, Copy, Clone)]
pub(crate) enum WalkEvent {
    Enter(CstNode),
    Exit(CstNode),
}

pub(crate) fn walk<'a>(doc: &'a TomlDoc, node: CstNode) -> impl Iterator<Item=WalkEvent> + 'a {
    let mut done = false;
    ::itertools::unfold(WalkEvent::Enter(node), move |pos| {
        if done {
            return None;
        }
        let res = *pos;
        *pos = match *pos {
            WalkEvent::Enter(node) => {
                match node.children(doc).first() {
                    Some(child) => {
                        WalkEvent::Enter(child)
                    },
                    None => WalkEvent::Exit(node),
                }
            }
            WalkEvent::Exit(node) => {
                match node.next_sibling(doc) {
                    Some(sibling) => WalkEvent::Enter(sibling),
                    None => match node.parent(doc) {
                        Some(node) => WalkEvent::Exit(node),
                        None => {
                            done = true;
                            WalkEvent::Exit(node)
                        }
                    }
                }
            }
        };
        Some(res)
    })
}
