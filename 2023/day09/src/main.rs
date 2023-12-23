use utils::read_lines;

fn main() {
    println!(
        "[PART 1]: {}",
        parse_sequences("./input.txt")
            .into_iter()
            .map(extrapolate)
            .sum::<i32>()
    );

    println!(
        "[PART 2]: {}",
        parse_sequences("./input.txt")
            .into_iter()
            .map(extrapolate_backwards)
            .sum::<i32>()
    );
}

fn extrapolate(ns: Vec<i32>) -> i32 {
    seq_diffs(ns, Vec::new())
        .iter()
        .rev()
        .fold(0, |acc, seq| seq.last().unwrap() + acc)
}

fn extrapolate_backwards(ns: Vec<i32>) -> i32 {
    seq_diffs(ns, Vec::new())
        .iter()
        .rev()
        .fold(0, |acc, seq| seq.first().unwrap() - acc)
}

fn seq_diffs(ns: Vec<i32>, mut acc: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if ns.iter().all(|&n| n == 0) {
        return acc;
    }

    acc.push(ns.clone());

    let diffs = ns.iter().skip(1).zip(&ns).map(|(n1, n0)| n1 - n0).collect();

    seq_diffs(diffs, acc)
}

fn parse_sequences(filename: &str) -> Vec<Vec<i32>> {
    read_lines(filename)
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            parse_sequences("./example.txt")
                .into_iter()
                .map(extrapolate)
                .sum::<i32>(),
            114
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            parse_sequences("./example.txt")
                .into_iter()
                .map(extrapolate_backwards)
                .sum::<i32>(),
            2
        )
    }
}
