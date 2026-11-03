use crate::trie::Node;

// struct MockNode {
//     value: String,
//     children: Vec<MockNode>,
// }


// fn mock_data() -> MockNode {
//     let alice = MockNode {
//         value: String::from("Alice"),
//         children: vec![],
//     };

//     let bob = MockNode {
//       value: String::from("Bob"),
//       children: vec![],
//     };

//     let tom = MockNode {
//       value: String::from("Tom"),
//       children: vec![],
//     };

//     let eight = MockNode {
//       value: String::from("8"),
//       children: vec![tom],
//     };

//     let six = MockNode {
//       value: String::from("6"),
//       children: vec![eight],
//     };

//     let two = MockNode {
//       value: String::from("2"),
//       children: vec![alice],
//     };

//     let nine = MockNode {
//       value: String::from("9"),
//       children: vec![bob],
//     };

//     let one = MockNode {
//       value: String::from("1"),
//       children: vec![two, nine],
//     };

//     let four = MockNode {
//       value: String::from("4"),
//       children: vec![one],
//     };

//     let zero = MockNode {
//       value: String::from("0"),
//       children: vec![four, six],
//     };

//     return zero;
// }

fn generate_line(node: &Node, depth: usize, result: &mut String) {
    let stars = "*".repeat(depth);

    result.push_str(&stars);
    result.push_str(" ");
    result.push_str(node.value());
    result.push_str("\n");

    for child in node.children() {
        generate_line(child, depth + 1, result);
    }
}

fn generate_plantuml(trees: &[Node]) -> String {
    let mut result = String::new();

    result.push_str("@startmindmap\n");

    for tree in trees {
        generate_line(tree, 1, &mut result);
    }

    result.push_str("@endmindmap\n");

    return result;
}

pub fn write_plantuml(trees: &[Node]) {
    let create_result = std::fs::create_dir_all("output");

    match create_result {
        Ok(_) => println!("Dossier output prêt"),
        Err(error) => {
            println!("Erreur : {}", error);
            return;
        }
    }

    let content = generate_plantuml(trees);

    let write_result = std::fs::write("output/output.puml", content);

    match write_result {
        Ok(_) => println!("Fichier PlantUML créé"),
        Err(error) => println!("Erreur : {}", error),
    }
}

