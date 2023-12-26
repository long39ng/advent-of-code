use std::{collections::HashMap, isize};
use utils::read_lines;

fn main() {
    let (graph, start) = parse_graph("./input.txt");

    let cycle_nodes = traverse_cycle(&graph, start);

    println!("[PART 1]: {}", cycle_nodes.len() / 2);

    println!("[PART 2]: {}", count_points_inside(&cycle_nodes));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile(usize, usize);

type Graph = HashMap<Tile, [Tile; 2]>;

fn traverse_cycle(graph: &Graph, start: Tile) -> Vec<Tile> {
    let mut nodes = vec![start];

    let next_node = *graph
        .iter()
        // Pick one of two adjacent nodes to start traversing
        .find(|(_, nbs)| nbs.contains(&start))
        .map(|(v, _)| v)
        .unwrap();

    dfs_cycle(graph, start, next_node, start, &mut nodes);

    nodes
}

fn dfs_cycle(graph: &Graph, start: Tile, node: Tile, prev: Tile, visited: &mut Vec<Tile>) {
    visited.push(node);

    // Only one possible next node
    let next = *graph[&node].iter().find(|&&v| v != prev).unwrap();

    if next != start {
        dfs_cycle(graph, start, next, node, visited)
    }
}

// Pick's theorem
fn count_points_inside(nodes: &[Tile]) -> usize {
    polygon_area(nodes) + 1 - nodes.len() / 2
}

// Shoelace formula (triangle formula)
fn polygon_area(nodes: &[Tile]) -> usize {
    (0..nodes.len())
        .fold(0, |acc, i| {
            let Tile(x0, y0) = nodes[i];
            let Tile(x1, y1) = nodes[(i + 1) % nodes.len()];

            acc + (x0 * y1) as isize - (x1 * y0) as isize
        })
        .abs() as usize
        / 2
}

fn parse_graph(filename: &str) -> (Graph, Tile) {
    let lines: Vec<_> = read_lines(filename)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let n_rows = lines.len();
    let n_cols = lines[0].len();

    let mut graph: Graph = HashMap::new();

    let mut start = Tile(0, 0);

    lines.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let node = Tile(i, j);

            if let Some(((i0, j0), (i1, j1))) = match c {
                '|' if i > 0 && i < n_rows - 1 => Some(((i - 1, j), (i + 1, j))),
                '-' if j > 0 && j < n_cols - 1 => Some(((i, j - 1), (i, j + 1))),
                'L' if i > 0 && j < n_cols - 1 => Some(((i - 1, j), (i, j + 1))),
                'J' if i > 0 && j > 0 => Some(((i - 1, j), (i, j - 1))),
                '7' if i < n_rows - 1 && j > 0 => Some(((i, j - 1), (i + 1, j))),
                'F' if i < n_rows - 1 && j < n_cols - 1 => Some(((i, j + 1), (i + 1, j))),
                'S' => {
                    start = node;
                    None
                }
                _ => None,
            } {
                graph.insert(node, [Tile(i0, j0), Tile(i1, j1)]);
            }
        });
    });

    (graph, start)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let (graph1, start1) = parse_graph("./example-1.txt");
        let (graph2, start2) = parse_graph("./example-2.txt");

        assert_eq!(
            (
                traverse_cycle(&graph1, start1).len() / 2,
                traverse_cycle(&graph2, start2).len() / 2
            ),
            (4, 8)
        )
    }

    #[test]
    fn part2() {
        let examples = vec![
            "./example-3.txt",
            "./example-4.txt",
            "./example-5.txt",
            "./example-6.txt",
        ];

        let results: Vec<_> = examples
            .iter()
            .map(|&example| {
                let (graph, start) = parse_graph(example);
                count_points_inside(&traverse_cycle(&graph, start))
            })
            .collect();

        assert_eq!(results, vec![4, 4, 8, 10]);
    }
}
