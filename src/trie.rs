#[derive(Debug)]
enum NodeKind {
    Root,
    Digit,
    Name,
}

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    value: String,
    children: Vec<Node>,
}

impl Node {
    fn new(kind: NodeKind, value: &str) -> Node {
        Node {
            kind,
            value: String::from(value),
            children: Vec::new(),
        }
    }

    pub fn root() -> Node {
        Node::new(NodeKind::Root, "")
    }

    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = self;

        for digit in number.chars() {
            current.children.push(Node::new(NodeKind::Digit, &digit.to_string()));
            let last = current.children.len() - 1;
            current = &mut current.children[last];
        }

        current.children.push(Node::new(NodeKind::Name, name));
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

    #[test]
    fn insert_multi_digit_number_should_create_one_node_per_digit() {
        let mut root = Node::root();

        root.insert("15", "SAMU");

        let first = &root.children[0];
        assert_eq!(first.value, "1");
        let second = &first.children[0];
        assert_eq!(second.value, "5");
        assert_eq!(second.children[0].value, "SAMU");
    }
}
