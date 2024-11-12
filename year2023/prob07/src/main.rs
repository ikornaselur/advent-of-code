use advent::prelude::*;
use std::fmt;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = match self {
            HandType::FiveOfAKind => 8,
            HandType::FourOfAKind => 7,
            HandType::FullHouse => 6,
            HandType::ThreeOfAKind => 5,
            HandType::TwoPair => 4,
            HandType::OnePair => 3,
            HandType::HighCard => 2,
        };
        let other_value = match other {
            HandType::FiveOfAKind => 8,
            HandType::FourOfAKind => 7,
            HandType::FullHouse => 6,
            HandType::ThreeOfAKind => 5,
            HandType::TwoPair => 4,
            HandType::OnePair => 3,
            HandType::HighCard => 2,
        };
        self_value.cmp(&other_value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u8);

impl Card {
    fn new(card: char) -> Self {
        match card {
            '*' => Self(1), // A part 2 joker
            'T' => Self(10),
            'J' => Self(11),
            'Q' => Self(12),
            'K' => Self(13),
            'A' => Self(14),
            '2'..='9' => Self(card.to_digit(10).unwrap() as u8),
            _ => panic!("Invalid card: {}", card),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            14 => write!(f, "A"),
            10 => write!(f, "T"),
            11 => write!(f, "J"),
            12 => write!(f, "Q"),
            13 => write!(f, "K"),
            _ => write!(f, "{}", self.0),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand {
    fn new(hand: &str) -> Self {
        if hand.len() != 5 {
            panic!("Invalid hand: {}", hand);
        }
        let mut cards = [Card(0); 5];
        for (i, card) in hand.chars().enumerate() {
            cards[i] = Card::new(card);
        }
        Self(cards)
    }

    fn r#type(&self) -> HandType {
        let mut counts: HashMap<u8, u8> = self.0.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card.0).or_insert(0) += 1;
            map
        });

        let jokers = counts.remove(&1).unwrap_or(0);
        let sorts = counts.values().len();

        if jokers == 5 || sorts == 1 {
            return HandType::FiveOfAKind;
        }
        if sorts == 2 {
            if counts.values().any(|&v| v == (4 - jokers)) {
                return HandType::FourOfAKind;
            }
            if counts.values().any(|&v| v == (3 - jokers)) {
                // If we have 3 of a sort, then it doesn't matter if we have a joker or not, it's a
                // full house
                return HandType::FullHouse;
            }
        }
        if sorts == 3 {
            if counts.values().any(|&v| v == (3 - jokers)) {
                return HandType::ThreeOfAKind;
            }
            if counts.values().any(|&v| v == (2 - jokers)) {
                return HandType::TwoPair;
            }
        }
        if sorts == 4 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.r#type() != other.r#type() {
            self.r#type().cmp(&other.r#type())
        } else {
            self.0.cmp(&other.0)
        }
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    // Each line is of the form: <cards> <score>, such as:
    //
    // 32T3K 123
    //
    // meaning the hand 32T3K with the score 123
    //
    // We'll parse the hands into a vector of (Hand, u32) tuples
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let hand = Hand::new(parts.next().unwrap());
            let score = parts.next().unwrap().parse::<u32>().unwrap();
            (hand, score)
        })
        .collect();

    // Then we sort it by the hand
    hands.sort();

    // And finally we can sum up the scores multiplied by the rank (idx 0 is rank 1)
    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, (_, score))| (idx + 1) as u32 * score)
        .sum();

    Ok(sum)
}

fn part2(input: &str) -> Result<u32> {
    // Replace the J with a * for part 2
    part1(&input.replace('J', "*"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 5905);
    }

    #[test]
    fn test_card_new() {
        assert_eq!(Card::new('A'), Card(14));
        assert_eq!(Card::new('T'), Card(10));
        assert_eq!(Card::new('3'), Card(3));
        assert!(std::panic::catch_unwind(|| Card::new('1')).is_err());
    }

    #[test]
    fn test_hand_new() {
        let hand = Hand::new("27TKQ");
        assert_eq!(hand.0[0], Card(2));
        assert_eq!(hand.0[1], Card(7));
        assert_eq!(hand.0[2], Card(10));
        assert_eq!(hand.0[3], Card(13));
        assert_eq!(hand.0[4], Card(12));
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(Hand::new("27TKQ").r#type(), HandType::HighCard);
        assert_eq!(Hand::new("27T2Q").r#type(), HandType::OnePair);
        assert_eq!(Hand::new("27T22").r#type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::new("77T22").r#type(), HandType::TwoPair);
        assert_eq!(Hand::new("77722").r#type(), HandType::FullHouse);
        assert_eq!(Hand::new("77772").r#type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("77777").r#type(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_hand_ord() {
        assert!(Hand::new("45678") > Hand::new("23456"));
        assert!(Hand::new("45678") < Hand::new("22456"));
        assert!(Hand::new("32T3K") < Hand::new("T55J5"));
        assert!(Hand::new("33332") > Hand::new("2AAAA"));

        let mut hands = vec![
            Hand::new("32T3K"),
            Hand::new("T55J5"),
            Hand::new("KK677"),
            Hand::new("KTJJT"),
            Hand::new("QQQJA"),
        ];
        hands.sort();
        assert_eq!(
            hands,
            vec![
                Hand::new("32T3K"),
                Hand::new("KTJJT"),
                Hand::new("KK677"),
                Hand::new("T55J5"),
                Hand::new("QQQJA"),
            ]
        );
    }

    #[test]
    fn test_hand_type_with_joker() {
        // Five of a kind
        assert_eq!(Hand::new("7777*").r#type(), HandType::FiveOfAKind);
        assert_eq!(Hand::new("7*77*").r#type(), HandType::FiveOfAKind);
        assert_eq!(Hand::new("7**7*").r#type(), HandType::FiveOfAKind);
        assert_eq!(Hand::new("*7***").r#type(), HandType::FiveOfAKind);
        assert_eq!(Hand::new("*****").r#type(), HandType::FiveOfAKind);

        // Four of a kind
        assert_eq!(Hand::new("3777*").r#type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("377**").r#type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("37***").r#type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("3****").r#type(), HandType::FiveOfAKind); // Better than 4 of a kind

        // Full house
        assert_eq!(Hand::new("3377*").r#type(), HandType::FullHouse);
        assert_eq!(Hand::new("337**").r#type(), HandType::FourOfAKind); // Better than full house

        // Three of a kind
        assert_eq!(Hand::new("3327*").r#type(), HandType::ThreeOfAKind);

        // Two pair
        // Is there any way to have a joker that turns into two pairs? With something like
        // 2234* you would make the joker be a 2 for a three of a kind rather than two pair

        // One pair
        assert_eq!(Hand::new("2345*").r#type(), HandType::OnePair);
    }
}
