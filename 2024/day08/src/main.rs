use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use utils::read_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (antennas, dims) = read_map("input.txt")?;

    println!("[PART 1] {}", count_antinodes(&antennas, &dims, false));

    println!("[PART 2] {}", count_antinodes(&antennas, &dims, true));

    Ok(())
}

type AntennaLocations = HashMap<char, Vec<(usize, usize)>>;
type AntennaMap = (AntennaLocations, (usize, usize));

fn count_antinodes(
    antennas: &AntennaLocations,
    dims: &(usize, usize),
    include_lines: bool,
) -> usize {
    antennas
        .iter()
        .flat_map(|(_, locations)| {
            locations.iter().combinations(2).flat_map(|pair| {
                if include_lines {
                    line_antinodes(pair[0], pair[1], dims)
                } else {
                    pair_antinodes(pair[0], pair[1], dims)
                }
            })
        })
        .unique()
        .count()
}

fn pair_antinodes(
    a: &(usize, usize),
    b: &(usize, usize),
    dims: &(usize, usize),
) -> Vec<(usize, usize)> {
    let diffs = (a.0.abs_diff(b.0), a.1.abs_diff(b.1));

    let delta_signs = match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
        (Ordering::Less, Ordering::Less) => [(-1, -1), (1, 1)],
        (Ordering::Less, _) => [(-1, 1), (1, -1)],
        (_, Ordering::Less) => [(1, -1), (-1, 1)],
        (_, _) => [(1, 1), (-1, -1)],
    };

    [a, b]
        .into_iter()
        .zip(delta_signs)
        .filter_map(|(base, signs)| {
            base.0
                .checked_add_signed(signs.0 * diffs.0 as isize)
                .zip(base.1.checked_add_signed(signs.1 * diffs.1 as isize))
        })
        .filter(|&(i, j)| i < dims.0 && j < dims.1)
        .collect()
}

fn line_antinodes(
    a: &(usize, usize),
    b: &(usize, usize),
    dims: &(usize, usize),
) -> Vec<(usize, usize)> {
    let diffs = (a.0.abs_diff(b.0), a.1.abs_diff(b.1));

    let upper_is = (-(a.0 as isize)..=0)
        .step_by(diffs.0)
        .map(|i| (-i) as usize);

    let lower_is = (a.0..dims.0).step_by(diffs.0);

    let left_js = (-(a.1 as isize)..=0)
        .step_by(diffs.1)
        .map(|j| (-j) as usize);

    let right_js = (a.1..dims.1).step_by(diffs.1);

    if a.0.cmp(&b.0) == a.1.cmp(&b.1) {
        upper_is
            .zip(left_js)
            .chain(lower_is.zip(right_js).skip(1))
            .collect()
    } else {
        upper_is
            .zip(right_js)
            .chain(lower_is.zip(left_js).skip(1))
            .collect()
    }
}

fn read_map(input_path: &str) -> std::io::Result<AntennaMap> {
    let lines: Vec<Vec<_>> = read_lines(input_path)?
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let dims = (lines.len() as usize, lines[0].len() as usize);

    let mut antennas: AntennaLocations = HashMap::new();

    lines.iter().enumerate().for_each(|(i, cs)| {
        cs.iter().enumerate().for_each(|(j, &c)| {
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|locations| locations.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        })
    });

    Ok((antennas, dims))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let (antennas, dims) = read_map("example.txt")?;
        assert_eq!(count_antinodes(&antennas, &dims, false), 14);
        assert_eq!(count_antinodes(&antennas, &dims, true), 34);
        Ok(())
    }
}
