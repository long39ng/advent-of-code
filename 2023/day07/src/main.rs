use std::cmp::Ordering;
use std::collections::HashMap;
use utils::read_lines;

fn main() {
    println!(
        "[PART 1]: {}",
        total_winnings(&mut parse_list_of_hands("./input.txt", false))
    );

    println!(
        "[PART 2]: {}",
        total_winnings(&mut parse_list_of_hands("./input.txt", true))
    );
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        HandType::from_hand(self)
            .cmp(&HandType::from_hand(other))
            // array implements lexicographical comparison
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HandType {
    fn from_hand(hand: &Hand) -> HandType {
        let mut card_counts: HashMap<&Card, u8> = HashMap::new();

        for card in &hand.cards {
            *card_counts.entry(card).or_insert(0) += 1
        }

        let n_jokers = card_counts.remove(&Card::Joker).unwrap_or(0);

        let max_count = card_counts.values().max().unwrap_or(&0) + n_jokers;

        match card_counts.len() {
            0..=1 => HandType::FiveOfAKind, // 0 if all cards are jokers
            2 if max_count == 4 => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if max_count == 3 => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Card {
    fn from_char(c: char, with_jokers: bool) -> Card {
        match c {
            'J' if with_jokers => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}

fn total_winnings(list_of_hands: &mut Vec<(Hand, u32)>) -> u32 {
    list_of_hands.sort_by(|(a_hand, _), (b_hand, _)| a_hand.cmp(b_hand));

    list_of_hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| *bid * (rank as u32 + 1))
        .sum()
}

fn parse_list_of_hands(filename: &str, with_jokers: bool) -> Vec<(Hand, u32)> {
    read_lines(filename)
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| {
            let [hand_part, bid_part]: [&str; 2] = line
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let hand = Hand {
                cards: hand_part
                    .chars()
                    .map(|c| Card::from_char(c, with_jokers))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            };

            let bid = bid_part.parse().unwrap();

            (hand, bid)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            total_winnings(&mut parse_list_of_hands("./example.txt", false)),
            6440
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            total_winnings(&mut parse_list_of_hands("./example.txt", true)),
            5905
        )
    }
}
