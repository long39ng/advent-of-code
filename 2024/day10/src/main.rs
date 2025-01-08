use num_complex::Complex;
use std::collections::{HashMap, HashSet};
use utils::read_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topo = read_topo("input.txt")?;
    println!("[PART 1] {}", sum_trailhead_scores(&topo, true));
    println!("[PART 2] {}", sum_trailhead_scores(&topo, false));
    Ok(())
}

fn trailhead_score_dfs(
    next_cell: Complex<i32>,
    next_height: u8,
    topo: &HashMap<Complex<i32>, u8>,
    reached_peaks: &mut HashSet<Complex<i32>>,
    ignore_same_peak_trails: bool,
) -> u32 {
    if !topo.contains_key(&next_cell)
        || topo[&next_cell] != next_height
        || ignore_same_peak_trails && reached_peaks.contains(&next_cell)
    {
        return 0;
    }
    if next_height == 9 {
        reached_peaks.insert(next_cell);
        return 1;
    }
    let rook_deltas = [
        Complex::new(1, 0),
        Complex::new(-1, 0),
        Complex::new(0, 1),
        Complex::new(0, -1),
    ];
    rook_deltas
        .iter()
        .map(|d| {
            trailhead_score_dfs(
                next_cell + d,
                next_height + 1,
                topo,
                reached_peaks,
                ignore_same_peak_trails,
            )
        })
        .sum()
}

fn sum_trailhead_scores(topo: &HashMap<Complex<i32>, u8>, ignore_same_peak_trails: bool) -> u32 {
    topo.iter()
        .filter(|(_, &height)| height == 0)
        .map(|(&cell, _)| {
            let mut reached_peaks = HashSet::new();

            trailhead_score_dfs(cell, 0, topo, &mut reached_peaks, ignore_same_peak_trails)
        })
        .sum()
}

fn read_topo(input_path: &str) -> std::io::Result<HashMap<Complex<i32>, u8>> {
    Ok(read_lines(input_path)?
        .map_while(Result::ok)
        .map(|row| row.chars().collect::<Vec<_>>())
        .enumerate()
        .flat_map(|(i, cs)| {
            cs.into_iter().enumerate().filter_map(move |(j, c)| {
                c.to_digit(10)
                    .map(|z| (Complex::new(i as i32, j as i32), z as u8))
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let topo = read_topo("example.txt")?;
        assert_eq!(sum_trailhead_scores(&topo, true), 36);
        assert_eq!(sum_trailhead_scores(&topo, false), 81);
        Ok(())
    }
}
