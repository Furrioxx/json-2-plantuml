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

    #[test]
    fn insert_one_digit_number_should_create_digit_then_name() {
        let mut root = Node::root();

        root.insert("7", "Police");

        assert_eq!(root.children.len(), 1);
        let digit = &root.children[0];
        match digit.kind {
            NodeKind::Digit => {}
            _ => panic!("first node should be of kind Digit"),
        }
        assert_eq!(digit.value, "7");
        let name = &digit.children[0];
        match name.kind {
            NodeKind::Name => {}
            _ => panic!("last node should be of kind Name"),
        }
        assert_eq!(name.value, "Police");
    }
}
