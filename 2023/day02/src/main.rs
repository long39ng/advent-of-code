use std::collections::HashMap;
use utils::read_lines;

fn main() {
    if let Some(sum1) = answer("./input.txt", sum_possible_ids) {
        println!("[PART 1] Sum of IDs of possible games: {}", sum1)
    }

    if let Some(sum1) = answer("./input.txt", sum_power) {
        println!("[PART 2] Sum of IDs of possible games: {}", sum1)
    }
}

struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

fn answer(input_path: &str, summariser: fn(Vec<Game>) -> usize) -> Option<usize> {
    if let Ok(lines) = read_lines(input_path) {
        let games: Vec<Game> = lines
            .filter_map(|line| line.ok())
            .map(|line| parse_game(&line))
            .collect();

        Some(summariser(games))
    } else {
        None
    }
}

fn sum_possible_ids(games: Vec<Game>) -> usize {
    games
        .into_iter()
        .filter(|game| game_is_possible(game))
        .map(|game| game.id)
        .sum()
}

fn game_is_possible(game: &Game) -> bool {
    game.sets
        .iter()
        .all(|set| set.red <= BAG.red && set.green <= BAG.green && set.blue <= BAG.blue)
}

fn sum_power(games: Vec<Game>) -> usize {
    games
        .iter()
        .map(|game| max_cube_set(&game.sets))
        .map(|set| power_cube_set(set))
        .sum()
}

fn power_cube_set(set: CubeSet) -> usize {
    set.red * set.green * set.blue
}

fn max_cube_set(sets: &Vec<CubeSet>) -> CubeSet {
    let (max_red, max_green, max_blue) = sets.iter().fold((0, 0, 0), |(red, green, blue), set| {
        (red.max(set.red), green.max(set.green), blue.max(set.blue))
    });

    CubeSet {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

fn parse_game(line: &str) -> Game {
    let parts: Vec<&str> = line.split(':').collect();

    Game {
        id: parts[0].split(' ').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
        sets: parts[1]
            .trim()
            .split(';')
            .map(|s| parse_cube_set(s))
            .collect(),
    }
}

fn parse_cube_set(s: &str) -> CubeSet {
    let counts: HashMap<&str, usize> = s
        .split(',')
        .map(|count_str| {
            let parts: Vec<&str> = count_str.trim().split(' ').collect();

            (parts[1], parts[0].parse().unwrap())
        })
        .collect();

    CubeSet {
        red: *counts.get("red").unwrap_or(&0),
        green: *counts.get("green").unwrap_or(&0),
        blue: *counts.get("blue").unwrap_or(&0),
    }
}

const BAG: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(answer("./example.txt", sum_possible_ids), Some(8))
    }

    #[test]
    fn part2() {
        assert_eq!(answer("./example.txt", sum_power), Some(2286))
    }
}
