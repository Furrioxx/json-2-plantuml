struct MockNode {
    value: String,
    children: Vec<MockNode>,
}


fn mock_data() -> MockNode {
    let alice = MockNode {
        value: String::from("Alice"),
        children: vec![],
    };

    let bob = MockNode {
      value: String::from("Bob"),
      children: vec![],
    };

    let tom = MockNode {
      value: String::from("Tom"),
      children: vec![],
    };

    let eight = MockNode {
      value: String::from("8"),
      children: vec![tom],
    };

    let six = MockNode {
      value: String::from("6"),
      children: vec![eight],
    };

    let two = MockNode {
      value: String::from("2"),
      children: vec![alice],
    };

    let nine = MockNode {
      value: String::from("9"),
      children: vec![bob],
    };

    let one = MockNode {
      value: String::from("1"),
      children: vec![two, nine],
    };

    let four = MockNode {
      value: String::from("4"),
      children: vec![one],
    };

    let zero = MockNode {
      value: String::from("0"),
      children: vec![four, six],
    };

    return zero;
}

// ici on utilise &MockNode car on veut pas que la fonction ne prenne pas la propriété de l'objet, on veut juste lire les données et pour depth on prend usize car c'est un entier positif qui repésente la profondeur.
fn generate_line(node: &MockNode, depth: usize) {
    let stars = "*".repeat(depth);
    println!("{} {}", stars, node.value);

    for child in &node.children {
        generate_line(child, depth + 1);
    }
}

pub fn test_mock() {
    let tree = mock_data();

    generate_line(&tree, 1);
}
