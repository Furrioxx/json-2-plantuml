use crate::trie::Node;

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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::trie::insert;

    #[test]
    fn test_generate_empty_plantuml() {
        let trees: Vec<Node> = Vec::new();

        let result = generate_plantuml(&trees);

        let expected = "@startmindmap\n@endmindmap\n";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_single_number() {
        let mut trees: Vec<Node> = Vec::new();

        insert(&mut trees, "15", "SAMU");

        let result = generate_plantuml(&trees);

        let expected = concat!(
            "@startmindmap\n",
            "* 1\n",
            "** 5\n",
            "*** SAMU\n",
            "@endmindmap\n"
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_shared_prefix() {
        let mut trees: Vec<Node> = Vec::new();

        insert(&mut trees, "15", "SAMU");
        insert(&mut trees, "17", "Police");

        let result = generate_plantuml(&trees);

        let expected = concat!(
            "@startmindmap\n",
            "* 1\n",
            "** 5\n",
            "*** SAMU\n",
            "** 7\n",
            "*** Police\n",
            "@endmindmap\n"
        );

        assert_eq!(result, expected);
    }
}

