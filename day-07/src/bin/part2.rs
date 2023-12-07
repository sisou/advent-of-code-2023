fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    cards: Vec<u8>,
    type_: HandType,
    bet: u32,
}

impl Hand {
    pub fn from_line(line: &str) -> Self {
        let card_values = "J23456789TQKA";

        let parts: Vec<_> = line.split_whitespace().collect();

        let cards: Vec<_> = parts[0]
            .chars()
            .map(|c| card_values.find(c).unwrap() as u8)
            .collect();
        let bet = parts[1].parse().unwrap();

        let type_ = Hand::classify(&cards);

        Hand { cards, type_, bet }
    }

    fn classify(cards: &[u8]) -> HandType {
        let mut counts = [0u8; 15];
        for &card in cards {
            counts[card as usize] += 1;
        }

        // Pop off the jokers - the match statement below automatically produces the right result,
        // because any jokers automatically count towards the highest count, which is never used.
        let counts = &counts[1..];

        let mut counts = counts.iter().filter(|c| **c > 0).collect::<Vec<_>>();
        counts.sort_unstable();

        match counts.len() {
            0 => HandType::FiveOfAKind,
            1 => HandType::FiveOfAKind,
            2 => {
                if *counts[0] == 1 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if *counts[0] == 1 && *counts[1] == 1 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::Pair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.type_.cmp(&other.type_) {
            std::cmp::Ordering::Equal => {
                // Compare by cards, first higher card wins
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match a.cmp(b) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                std::cmp::Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

// Implement sort for Hand
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn run(input: &str) -> String {
    let mut hands: Vec<_> = input.lines().map(Hand::from_line).collect();

    hands.sort_unstable();

    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        total += (i as u32 + 1) * hand.bet;
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(result, "5905")
    }
}
