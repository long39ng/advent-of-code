use std::collections::HashSet;
use utils::read_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reports = read_reports("input.txt")?;

    println!("[PART 1] {}", n_safe(&reports));

    println!("[PART 2] {}", n_safe_dampened(&reports));

    Ok(())
}

fn n_safe(reports: &[Vec<u32>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn n_safe_dampened(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_dampened(report))
        .count()
}

fn is_safe(report: &[u32]) -> bool {
    let diff_set: HashSet<i32> = report
        .windows(2)
        .map(|pair| pair[0] as i32 - pair[1] as i32)
        .collect();

    diff_set.is_subset(&HashSet::from([1, 2, 3]))
        || diff_set.is_subset(&HashSet::from([-1, -2, -3]))
}

fn is_safe_dampened(report: &[u32]) -> bool {
    is_safe(report)
        || (0..report.len()).any(|i| {
            let (head, tail) = report.split_at(i);
            is_safe(&[head, tail.split_at(1).1].concat())
        })
}

fn read_reports(input_path: &str) -> std::io::Result<Vec<Vec<u32>>> {
    Ok(read_lines(input_path)?
        .map_while(Result::ok)
        .map(|report| {
            report
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let reports = read_reports("example.txt")?;
        assert_eq!(n_safe(&reports), 2);
        assert_eq!(n_safe_dampened(&reports), 4);
        Ok(())
    }
}
