#[derive(Debug)]
enum NodeKind {
    Root,
}

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    value: String,
    children: Vec<Node>,
}

impl Node {
    pub fn root() -> Self {
        Self {
            kind: NodeKind::Root,
            value: String::new(),
            children: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_should_be_empty() {
        let root = Node::root();

        match root.kind {
            NodeKind::Root => {}
            _ => panic!("root should be of kind Root"),
        }
        assert_eq!(root.value, "");
        assert!(root.children.is_empty());
    }
}
