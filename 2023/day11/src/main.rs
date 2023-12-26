use itertools::Itertools;
use std::collections::HashSet;
use utils::read_lines;

fn main() {
    let galaxies = parse_grid("./input.txt");

    println!("[PART 1]: {}", galaxies.total_distance(2));

    println!("[PART 2]: {}", galaxies.total_distance(1_000_000));
}

type Cell = (usize, usize);

struct SparseGrid {
    cells: Vec<Cell>,
    n_rows: usize,
    n_cols: usize,
}

impl SparseGrid {
    fn from_rows(rows: Vec<Vec<bool>>) -> Self {
        let n_rows = rows.len();
        let n_cols = rows[0].len();

        let cells = rows
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, x)| if *x { Some((i, j)) } else { None })
                    .collect::<Vec<_>>()
            })
            .collect();

        SparseGrid {
            cells,
            n_rows,
            n_cols,
        }
    }

    fn empty_rows_cols(&self) -> (Vec<usize>, Vec<usize>) {
        let (nonempty_rows, nonempty_cols): (HashSet<_>, HashSet<_>) =
            self.cells.iter().cloned().unzip();

        let empty_rows = (0..self.n_rows)
            .filter(|i| !nonempty_rows.contains(i))
            .collect();
        let empty_cols = (0..self.n_cols)
            .filter(|j| !nonempty_cols.contains(j))
            .collect();

        (empty_rows, empty_cols)
    }

    fn total_distance(&self, space_expands_by: usize) -> usize {
        let (empty_rows, empty_cols) = self.empty_rows_cols();

        self.cells
            .iter()
            .combinations(2)
            .map(|pair| {
                let (a, b) = (pair[0], pair[1]);

                let empty_rows_between = empty_rows
                    .iter()
                    .filter(|&i| (a.0.min(b.0) + 1..a.0.max(b.0)).contains(i))
                    .count();

                let empty_cols_between = empty_cols
                    .iter()
                    .filter(|&j| (a.1.min(b.1) + 1..a.1.max(b.1)).contains(j))
                    .count();

                manhattan_distance(a, b)
                    + (empty_rows_between + empty_cols_between) * (space_expands_by - 1)
            })
            .sum()
    }
}

fn parse_grid(filename: &str) -> SparseGrid {
    SparseGrid::from_rows(
        read_lines(filename)
            .unwrap()
            .filter_map(Result::ok)
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
    )
}

fn manhattan_distance(a: &Cell, b: &Cell) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let galaxies = parse_grid("./example.txt");

        assert_eq!(galaxies.total_distance(2), 374);
    }

    #[test]
    fn part2() {
        let galaxies = parse_grid("./example.txt");

        assert_eq!(galaxies.total_distance(10), 1030);
        assert_eq!(galaxies.total_distance(100), 8410);
    }
}
