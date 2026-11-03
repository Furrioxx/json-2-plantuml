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

    fn is_digit(&self, value: &str) -> bool {
        match self.kind {
            NodeKind::Digit => self.value == value,
            _ => false,
        }
    }

    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = self;

        for digit in number.chars() {
            let value = digit.to_string();
            let mut position = None;

            for (index, child) in current.children.iter().enumerate() {
                if child.is_digit(&value) {
                    position = Some(index);
                }
            }

            let index = match position {
                Some(index) => index,
                None => {
                    current.children.push(Node::new(NodeKind::Digit, &value));
                    current.children.len() - 1
                }
            };

            current = &mut current.children[index];
        }

        current.children.push(Node::new(NodeKind::Name, name));
    }
}

pub fn insert(trees: &mut Vec<Node>, number: &str, name: &str) {
    let mut current = trees;

    for digit in number.chars() {
        let value = digit.to_string();
        let mut position = None;

        for (index, child) in current.iter().enumerate() {
            if child.is_digit(&value) {
                position = Some(index);
            }
        }

        let index = match position {
            Some(index) => index,
            None => {
                current.push(Node::new(NodeKind::Digit, &value));
                current.len() - 1
            }
        };

        current = &mut current[index].children;
    }

    current.push(Node::new(NodeKind::Name, name));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::import_json_file;

    fn find_name(root: &Node, number: &str) -> Option<String> {
        let mut current = root;

        for digit in number.chars() {
            let mut next = None;
            for child in &current.children {
                if child.is_digit(&digit.to_string()) {
                    next = Some(child);
                }
            }
            current = next?;
        }

        for child in &current.children {
            if let NodeKind::Name = child.kind {
                return Some(child.value.clone());
            }
        }
        None
    }

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
        let mut trees = Vec::new();

        insert(&mut trees, "7", "Police");

        assert_eq!(trees.len(), 1);
        let digit = &trees[0];
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

    #[test]
    fn insert_numbers_with_common_prefix_should_share_nodes() {
        let mut root = Node::root();

        root.insert("15", "SAMU");
        root.insert("17", "Police");

        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].children.len(), 2);
    }

    #[test]
    fn insert_number_extending_another_should_keep_both_names() {
        let mut root = Node::root();

        root.insert("15", "SAMU");
        root.insert("158", "Test");

        let five = &root.children[0].children[0];
        assert_eq!(five.children[0].value, "SAMU");
        assert_eq!(five.children[1].value, "8");
    }

    #[test]
    fn insert_should_never_follow_a_name_node() {
        let mut root = Node::root();

        root.insert("1", "5");
        root.insert("15", "SAMU");

        let one = &root.children[0];
        assert_eq!(one.children.len(), 2);
        assert!(one.children[0].children.is_empty());
    }

    #[test]
    fn insert_contacts_from_json_should_find_each_name() {
        let contacts = import_json_file("01_subject_example").expect("subject example should load");
        let mut root = Node::root();

        for contact in &contacts {
            root.insert(contact.nb(), contact.name());
        }

        assert_eq!(root.children.len(), 2);
        for contact in &contacts {
            assert_eq!(
                find_name(&root, contact.nb()),
                Some(contact.name().to_string())
            );
        }
    }
}
