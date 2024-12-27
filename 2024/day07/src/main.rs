use utils::read_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let equations = read_equations("input.txt")?;

    let total_calibration_result1: i64 = equations
        .iter()
        .filter(|(test_val, nums)| has_equation(test_val, nums, false))
        .map(|(test_val, _)| test_val)
        .sum();

    println!("[PART 1] {}", total_calibration_result1);

    let total_calibration_result2: i64 = equations
        .iter()
        .filter(|(test_val, nums)| has_equation(test_val, nums, true))
        .map(|(test_val, _)| test_val)
        .sum();

    println!("[PART 2] {}", total_calibration_result2);

    Ok(())
}

fn has_equation(test_val: &i64, nums: &[i64], accepts_concat: bool) -> bool {
    if let Some((last, head)) = nums.split_last() {
        if head.is_empty() {
            last == test_val
        } else {
            test_val % last == 0 && has_equation(&(test_val / last), head, accepts_concat)
                || accepts_concat
                    && ends_with(test_val, last)
                    && has_equation(&(test_val / 10_i64.pow(digits(last))), head, accepts_concat)
                || has_equation(&(test_val - last), head, accepts_concat)
        }
    } else {
        false
    }
}

fn ends_with(a: &i64, b: &i64) -> bool {
    (a - b) % 10_i64.pow(digits(b)) == 0
}

fn digits(n: &i64) -> u32 {
    n.ilog10() + 1
}

fn read_equations(input_path: &str) -> std::io::Result<Vec<(i64, Vec<i64>)>> {
    Ok(read_lines(input_path)?
        .map_while(Result::ok)
        .map(|line| {
            let (lhs, rhs) = line.split_once(':').unwrap();
            let test_val = lhs.parse().unwrap();
            let nums = rhs.split_whitespace().map(|n| n.parse().unwrap()).collect();
            (test_val, nums)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let equations = read_equations("example.txt")?;

        let total_calibration_result1: i64 = equations
            .iter()
            .filter(|(test_val, nums)| has_equation(test_val, nums, false))
            .map(|(test_val, _)| test_val)
            .sum();
        assert_eq!(total_calibration_result1, 3749);

        let total_calibration_result2: i64 = equations
            .iter()
            .filter(|(test_val, nums)| has_equation(test_val, nums, true))
            .map(|(test_val, _)| test_val)
            .sum();
        assert_eq!(total_calibration_result2, 11387);

        Ok(())
    }
}
