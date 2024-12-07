use std::collections::HashMap;
use utils::read_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = read_columns("input.txt")?;

    println!("[PART 1] {}", sum_distance(&left, &right));

    println!("[PART 2] {}", sum_similarity(&left, &right));

    Ok(())
}

fn sum_distance(left: &[u32], right: &[u32]) -> u32 {
    left.iter().zip(right).map(|(a, b)| a.abs_diff(*b)).sum()
}

fn sum_similarity(left: &[u32], right: &[u32]) -> usize {
    let left_counts = count(left);
    let right_counts = count(right);

    left_counts
        .iter()
        .filter_map(|(&value, &left_count)| {
            right_counts
                .get(&value)
                .map(|&right_count| left_count * value as usize * right_count)
        })
        .sum()
}

fn count(values: &[u32]) -> HashMap<u32, usize> {
    let mut counts = HashMap::new();
    values.iter().for_each(|&value| {
        *counts.entry(value).or_insert(0) += 1;
    });
    counts
}

fn read_columns(input_path: &str) -> std::io::Result<(Vec<u32>, Vec<u32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    read_lines(input_path)?
        .map_while(Result::ok)
        .for_each(|line| {
            let mut values = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok());

            if let (Some(a), Some(b)) = (values.next(), values.next()) {
                left.push(a);
                right.push(b);
            }
        });

    left.sort_unstable();
    right.sort_unstable();

    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let (left, right) = read_columns("example.txt")?;
        assert_eq!(sum_distance(&left, &right), 11);
        assert_eq!(sum_similarity(&left, &right), 31);
        Ok(())
    }
}
