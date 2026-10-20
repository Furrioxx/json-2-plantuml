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
