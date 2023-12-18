// Inspired by https://github.com/steven-terrana/advent-of-code/tree/main/2023/day05

use utils::read_lines;

fn main() {
    println!(
        "[PART 1]: {}",
        calculate_lowest_location(&parse_almanac("./input.txt", false))
    );

    println!(
        "[PART 2]: {}",
        calculate_lowest_location(&parse_almanac("./input.txt", true))
    );
}

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<(u64, u64)>, // ([start, end))
    seed_location_map: Vec<MapRange>,
}

type MapRange = (u64, u64, i64); // ([start, end), shift)

fn calculate_lowest_location(
    Almanac {
        seed_ranges,
        seed_location_map,
    }: &Almanac,
) -> u64 {
    seed_ranges
        .iter()
        // Get smallest value of each intersection
        .flat_map(|&(seed_start, seed_end)| {
            seed_location_map
                .iter()
                .filter_map(move |&(map_range_start, map_range_end, shift)| {
                    if seed_start < map_range_end && seed_end > map_range_start {
                        Some((seed_start.max(map_range_start) as i64 + shift) as u64)
                    } else {
                        None
                    }
                })
        })
        .reduce(u64::min)
        .unwrap()
}

fn parse_almanac(filename: &str, seeds_are_ranges: bool) -> Almanac {
    let lines: Vec<_> = read_lines(filename)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let seed_ranges = parse_seeds(lines[0].split(':').nth(1).unwrap(), seeds_are_ranges);

    let maps = lines[2..].split(|line| line.is_empty()).map(parse_map);

    // Assumes maps are topologically sorted
    let mut seed_location_map = maps.reduce(|acc, map| compose_maps(&acc, &map)).unwrap();

    seed_location_map.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

    Almanac {
        seed_ranges,
        seed_location_map,
    }
}

fn parse_seeds(string: &str, seeds_are_ranges: bool) -> Vec<(u64, u64)> {
    let numbers = parse_numbers(string);

    if seeds_are_ranges {
        numbers
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
            .collect()
    } else {
        numbers.into_iter().map(|seed| (seed, seed + 1)).collect()
    }
}

// Try to compose each range in a with each range in b
fn compose_maps(a: &Vec<MapRange>, b: &Vec<MapRange>) -> Vec<MapRange> {
    a.iter()
        .flat_map(|a_range| {
            b.iter()
                .filter_map(|b_range| compose_ranges(a_range, b_range))
        })
        .collect()
}

// Map b onto a, identify intersection, add shifts
fn compose_ranges(a: &MapRange, b: &MapRange) -> Option<MapRange> {
    let b_start: u64 = (b.0 as i64).saturating_sub(a.2).try_into().unwrap_or(0);
    let b_end: u64 = (b.1 as i64).saturating_sub(a.2).try_into().unwrap_or(0);

    if a.0 < b_end && a.1 > b_start {
        Some((a.0.max(b_start), a.1.min(b_end), a.2 + b.2))
    } else {
        None
    }
}

fn parse_map(lines: &[String]) -> Vec<MapRange> {
    // Ignore map categories because maps in input are already sorted along path
    // let (source_category, destination_category) = lines[0]
    //     .split_whitespace()
    //     .next()
    //     .unwrap()
    //     .split_once("-to-")
    //     .unwrap();

    let mut ranges: Vec<MapRange> = lines
        .iter()
        .skip(1)
        .map(|line| {
            let [dst_start, src_start, length] = parse_numbers(line).try_into().unwrap();
            (
                src_start,
                src_start + length,
                dst_start as i64 - src_start as i64,
            )
        })
        .collect();

    ranges.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

    // Fill gaps with non-shifting ranges
    // Iterate from the end so insert does not affect elements not yet visited
    (1..ranges.len()).rev().for_each(|i| {
        let previous_end = ranges[i - 1].1;
        let next_start = ranges[i].0;

        if previous_end != next_start {
            ranges.insert(i, (previous_end, next_start, 0))
        }
    });

    let first_start = ranges.first().unwrap().0;
    if first_start != 0 {
        ranges.insert(0, (0, first_start, 0));
    }

    let last_end = ranges.last().unwrap().1;
    ranges.push((last_end, i64::MAX as u64, 0));

    ranges
}

fn parse_numbers(string: &str) -> Vec<u64> {
    string.split_whitespace().flat_map(str::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            calculate_lowest_location(&parse_almanac("./example.txt", false)),
            35
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            calculate_lowest_location(&parse_almanac("./example.txt", true)),
            46
        );
    }
}
