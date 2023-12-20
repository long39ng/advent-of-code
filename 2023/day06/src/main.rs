use utils::read_lines;

fn main() {
    println!(
        "[PART 1]: {}",
        parse_races("./input.txt")
            .into_iter()
            .map(ways_to_win)
            .product::<usize>()
    );

    println!("[PART 2]: {}", ways_to_win(parse_race2("./input.txt")));
}

fn ways_to_win((time, record_distance): (u64, u64)) -> usize {
    time as usize + 1
        - (0..time)
            .take_while(|&hold_duration| distance(time, hold_duration) <= record_distance)
            .count()
            * 2
}

fn distance(time: u64, hold_duration: u64) -> u64 {
    hold_duration * (time - hold_duration)
}

fn parse_races(filename: &str) -> Vec<(u64, u64)> {
    let [times, distances]: [Vec<u64>; 2] = read_lines(filename)
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .flat_map(str::parse)
                .collect()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    times.into_iter().zip(distances).collect()
}

fn parse_race2(filename: &str) -> (u64, u64) {
    let [time, distance]: [u64; 2] = read_lines(filename)
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(String::from)
                .reduce(|acc, s| acc + &s)
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            parse_races("./example.txt")
                .into_iter()
                .map(ways_to_win)
                .product::<usize>(),
            288
        )
    }

    #[test]
    fn part2() {
        assert_eq!(ways_to_win(parse_race2("./example.txt")), 71503)
    }
}
