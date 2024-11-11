use std::collections::HashMap;

/// ```
/// assert_eq!("RRHTRHGMTTKM@", day06::solve(include_str!("input1.txt"), |node| node));
/// assert_eq!("RFJXQDCRHT@",   day06::solve(include_str!("input2.txt"), |node| &node[0..1]));
/// assert_eq!("RDJZQFMJSTBD@", day06::solve(include_str!("input3.txt"), |node| &node[0..1]));
/// ```
pub fn solve(input: &str, describe: fn(&str) -> &str) -> String {
    let mut child_to_parent = HashMap::new();
    let mut leaves = Vec::new();

    for line in input.lines() {
        let (parent, children) = line.split_once(':').unwrap();
        for child in children.split(',') {
            if child == "@" {
                leaves.push(parent);
            } else {
                child_to_parent.insert(child, parent);
            }
        }
    }

    let mut length_to_paths = HashMap::<usize, Vec<_>>::new();

    'leaf: for leaf in leaves {
        let mut path = vec!["@", leaf];

        while let Some(node) = child_to_parent.get(path.last().unwrap()) {
            if path.contains(node) {
                continue 'leaf; // Ignore cycles (for part III).
            }

            path.push(node);
        }

        length_to_paths.entry(path.len()).or_default().push(path);
    }

    let unique_length_path = &length_to_paths
        .values()
        .find(|paths| paths.len() == 1)
        .unwrap()[0];

    unique_length_path
        .iter()
        .rev()
        .copied()
        .map(describe)
        .collect()
}
