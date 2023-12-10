use std::collections::HashMap;
use utils::read_lines;

fn main() {
    let cards: Vec<_> = read_lines("./input.txt")
        .unwrap()
        .filter_map(Result::ok)
        .map(parse_card)
        .collect();

    println!("[PART 1] {}", sum_card_points(&cards));

    println!("[PART 2] {}", sum_card_copies(&cards));
}

struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

fn sum_card_points(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .filter_map(|card| count_matching_numbers(&card.numbers_you_have, &card.winning_numbers))
        .map(|n| 2u32.pow(n as u32 - 1))
        .sum()
}

fn sum_card_copies(cards: &Vec<Card>) -> u32 {
    let mut counts = HashMap::new();

    cards.iter().for_each(|card| {
        let this_count = counts.entry(card.id).or_insert(0);
        *this_count += 1;

        if let Some(copy_ids) = which_card_copies(card) {
            let this_count_clone = this_count.clone();

            copy_ids.iter().for_each(|id| {
                let copy_count = counts.entry(*id).or_insert(0);
                *copy_count += this_count_clone;
            })
        }
    });

    counts.values().sum()
}

fn which_card_copies(card: &Card) -> Option<Vec<usize>> {
    count_matching_numbers(&card.numbers_you_have, &card.winning_numbers)
        .map(|n_copies| (card.id + 1..=card.id + n_copies).collect())
}

fn count_matching_numbers(
    numbers_you_have: &Vec<u32>,
    winning_numbers: &Vec<u32>,
) -> Option<usize> {
    Some(
        numbers_you_have
            .iter()
            .filter(|x| winning_numbers.contains(&x))
            .count(),
    )
    .filter(|&n| n > 0)
}

fn parse_card(line: String) -> Card {
    let parts: Vec<_> = line.split(':').collect();

    let id = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();

    let numbers: Vec<_> = parts[1].split('|').map(parse_numbers).collect();

    let [winning_numbers, numbers_you_have] = numbers.try_into().unwrap();

    Card {
        id,
        winning_numbers,
        numbers_you_have,
    }
}

fn parse_numbers(string: &str) -> Vec<u32> {
    string
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example() -> Vec<Card> {
        read_lines("./example.txt")
            .unwrap()
            .filter_map(|line| line.ok())
            .map(parse_card)
            .collect()
    }

    #[test]
    fn part1() {
        assert_eq!(sum_card_points(&read_example()), 13)
    }

    #[test]
    fn part2() {
        assert_eq!(sum_card_copies(&read_example()), 30)
    }
}
