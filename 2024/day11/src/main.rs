use std::collections::HashMap;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let stones = read_stones("input.txt")?;
    let mut cache = HashMap::new();

    println!("[PART 1] {}", sum_stone_counts(&stones, 25, &mut cache));
    println!("[PART 2] {}", sum_stone_counts(&stones, 75, &mut cache));

    Ok(())
}

fn count_stones_from(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    } else if cache.contains_key(&(stone, blinks)) {
        return cache[&(stone, blinks)];
    }

    let n_out = if stone == 0 {
        count_stones_from(1, blinks - 1, cache)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let mid_10_pow = 10u64.pow(digits / 2);

            count_stones_from(stone / mid_10_pow, blinks - 1, cache)
                + count_stones_from(stone % mid_10_pow, blinks - 1, cache)
        } else {
            count_stones_from(stone * 2024, blinks - 1, cache)
        }
    };

    cache.insert((stone, blinks), n_out);

    n_out
}

fn sum_stone_counts(
    stones: &[u64],
    blinks: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    stones
        .iter()
        .map(|&stone| count_stones_from(stone, blinks, cache))
        .sum()
}

fn read_stones(input_path: &str) -> std::io::Result<Vec<u64>> {
    Ok(read_to_string(input_path)?
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> std::io::Result<()> {
        let stones = read_stones("example.txt")?;
        let mut cache = HashMap::new();
        assert_eq!(sum_stone_counts(&stones, 25, &mut cache), 55312);
        Ok(())
    }
}
