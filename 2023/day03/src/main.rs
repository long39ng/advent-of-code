use regex::Regex;
use std::collections::HashSet;
use utils::read_lines;

fn main() {
    let lines = read_lines("./input.txt")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    println!("[PART 1] {}", sum_part_numbers(&lines));

    println!("[PART 2] {}", sum_gear_ratios(&lines));
}

#[derive(PartialEq, Eq, Hash)]
struct Number {
    value: u32,
    cells: Vec<Cell>,
}

type Cell = (usize, usize);

fn sum_part_numbers(lines: &Vec<String>) -> u32 {
    let schematic = parse_symbols(lines);

    parse_numbers(lines)
        .iter()
        .filter(|number| {
            number.cells.iter().any(|cell| {
                lookaround(cell, &schematic).any(|(nb_i, nb_j)| schematic[nb_i][nb_j] != '.')
            })
        })
        .map(|number| number.value)
        .sum()
}

fn sum_gear_ratios(lines: &Vec<String>) -> u32 {
    let schematic = parse_symbols(lines);

    let numbers_adjacent_to_asterisk = parse_numbers(lines)
        .into_iter()
        .filter(|number| {
            number.cells.iter().any(|cell| {
                lookaround(cell, &schematic).any(|(nb_i, nb_j)| schematic[nb_i][nb_j] == '*')
            })
        })
        .collect();

    let asterisk_cells = schematic.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .filter(|&(_, &c)| c == '*')
            .map(move |(j, _)| (i, j))
    });

    asterisk_cells
        .filter_map(|cell| find_gear_ratio(&cell, &schematic, &numbers_adjacent_to_asterisk))
        .sum()
}

fn find_gear_ratio(cell: &Cell, schematic: &Vec<Vec<char>>, numbers: &Vec<Number>) -> Option<u32> {
    let adjacent_numbers: HashSet<_> = lookaround(cell, schematic)
        .flat_map(|nb| {
            numbers
                .iter()
                .filter(move |number| number.cells.contains(&nb))
        })
        .collect();

    adjacent_numbers
        .len()
        .eq(&2)
        .then(|| adjacent_numbers.iter().map(|number| number.value).product())
}

fn lookaround<'a>(
    (i, j): &'a Cell,
    schematic: &'a Vec<Vec<char>>,
) -> impl Iterator<Item = Cell> + 'a {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    // Assume nrow and ncol do not exceed usize::MAX
    .map(|(di, dj)| ((*i as isize + di) as usize, (*j as isize + dj) as usize))
    .filter(|(nb_i, nb_j)| nb_i < &schematic.len() && nb_j < &schematic[*nb_i].len())
}

fn parse_numbers(lines: &Vec<String>) -> Vec<Number> {
    let num_re = Regex::new(r"\d+").unwrap();

    lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            num_re.find_iter(&line).map(move |m| Number {
                value: m.as_str().parse().unwrap(),
                cells: m.range().map(|pos| (i, pos)).collect(),
            })
        })
        .collect()
}

fn parse_symbols(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                // Replace digits with .
                .map(|c| if c.is_digit(10) { '.' } else { c })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn read_example() -> Vec<String> {
        read_lines("./example.txt")
            .unwrap()
            .filter_map(|line| line.ok())
            .collect()
    }

    #[test]
    fn part1() {
        assert_eq!(sum_part_numbers(&read_example()), 4361)
    }

    #[test]
    fn part2() {
        assert_eq!(sum_gear_ratios(&read_example()), 467835)
    }
}
