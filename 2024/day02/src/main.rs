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
    let mut pair_iter = report.windows(2);
    if let Some(first_pair) = pair_iter.next() {
        let (first, second) = (first_pair[0], first_pair[1]);
        if first == second || first.abs_diff(second) > 3 {
            return false;
        }
        let direction = first.cmp(&second);

        return pair_iter.all(|pair| {
            let (a, b) = (pair[0], pair[1]);
            a.cmp(&b) == direction && a.abs_diff(b) <= 3
        });
    }
    true
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
