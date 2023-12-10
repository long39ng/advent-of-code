use utils::read_lines;

fn main() {
    if let Some(sum1) = sum_calibration_values("./input.txt", parse_line_1) {
        println!("[PART 1] {}", sum1);
    }

    if let Some(sum2) = sum_calibration_values("./input.txt", parse_line_2) {
        println!("[PART 2] {}", sum2);
    }
}

fn sum_calibration_values(input_path: &str, line_parser: fn(&str) -> Option<u32>) -> Option<u32> {
    if let Ok(lines) = read_lines(input_path) {
        let sum = lines
            .filter_map(|line| line.ok())
            .filter_map(|line| line_parser(&line))
            .sum();

        Some(sum)
    } else {
        None
    }
}

fn parse_line_1(line: &str) -> Option<u32> {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));

    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);

    Some(first_digit * 10 + last_digit)
}

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line_2(line: &str) -> Option<u32> {
    let first_digit = (0..line.len())
        .find_map(|pos| parse_digit_at(line, pos))
        .unwrap();

    let last_digit = (0..line.len())
        .rev()
        .find_map(|pos| parse_digit_at(line, pos))
        .unwrap();

    Some(first_digit * 10 + last_digit)
}

fn parse_digit_at(line: &str, pos: usize) -> Option<u32> {
    let char_at_pos = line.chars().nth(pos).unwrap();

    match char_at_pos.to_digit(10) {
        Some(digit) => Some(digit),
        None => parse_digit_word(&line[pos..]),
    }
}

fn parse_digit_word(s: &str) -> Option<u32> {
    DIGIT_WORDS
        .iter()
        .position(|&word| s.starts_with(word))
        .map(|index| (index + 1) as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            sum_calibration_values("./example-1.txt", parse_line_1),
            Some(142)
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            sum_calibration_values("./example-2.txt", parse_line_2),
            Some(281)
        )
    }
}
