use num::integer::lcm;
use std::collections::HashMap;
use utils::read_lines;

fn main() {
    let map = parse_map("./input.txt");

    println!(
        "[PART 1]: {}",
        count_steps(&map, "AAA", |label| label == "ZZZ")
    );

    println!(
        "[PART 2]: {}",
        map.1
            .keys()
            .filter(|label| label.ends_with('A'))
            .map(|start| count_steps(&map, start, |label| label.ends_with('Z')))
            .reduce(lcm)
            .unwrap()
    );
}

type Tree = HashMap<String, (String, String)>;

fn count_steps(
    (instructions, tree): &(Vec<char>, Tree),
    start: &str,
    stop_predicate: impl Fn(&str) -> bool,
) -> usize {
    instructions
        .iter()
        .cycle()
        .scan(start.to_string(), |node, direction| {
            *node = lookup(&tree, node, direction).unwrap();

            if stop_predicate(&node) {
                return None;
            }

            Some(node.clone())
        })
        .count()
        + 1
}

fn lookup(tree: &Tree, node: &String, direction: &char) -> Option<String> {
    tree.get(node).map(|children| match direction {
        'L' => children.0.clone(),
        'R' => children.1.clone(),
        _ => unreachable!(),
    })
}

fn parse_map(filename: &str) -> (Vec<char>, Tree) {
    let mut lines = read_lines(filename).unwrap().filter_map(Result::ok);

    let instruction = lines.next().unwrap().chars().collect();

    let tree = HashMap::from_iter(lines.skip(1).map(parse_line));

    (instruction, tree)
}

fn parse_line(line: String) -> (String, (String, String)) {
    let [label, children]: [&str; 2] = line
        .splitn(2, " = ")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let [left, right] = children
        .trim_matches(|c| c == '(' || c == ')')
        .split(", ")
        .map(String::from)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (label.to_string(), (left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            (
                count_steps(&parse_map("./example-1.txt"), "AAA", |label| label == "ZZZ"),
                count_steps(&parse_map("./example-2.txt"), "AAA", |label| label == "ZZZ")
            ),
            (2, 6)
        )
    }

    #[test]
    fn part2() {
        let map = parse_map("./example-3.txt");

        assert_eq!(
            map.1
                .keys()
                .filter(|label| label.ends_with('A'))
                .map(|start| count_steps(&map, start, |label| label.ends_with('Z')))
                .reduce(lcm)
                .unwrap(),
            6
        )
    }
}
