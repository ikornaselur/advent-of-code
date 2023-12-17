use advent::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

struct Card {
    winning_numbers: HashSet<u32>,
    playing_numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = AdventError;
    /// Create a new card from a string
    ///
    /// The string should be in this form:
    ///
    /// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    ///
    /// where the first 5 numbers left of the | are the winning numbers and on the right are the
    /// playing numbers
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.split_whitespace().collect::<Vec<_>>().join(" ");
        // Get everything after the first :
        let input = input.split(": ").collect::<Vec<_>>()[1];
        let mut numbers = input.split(" | ");
        let winning_numbers = numbers
            .next()
            .ok_or(AdventError::InvalidInput)?
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let playing_numbers = numbers
            .next()
            .ok_or(AdventError::InvalidInput)?
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            winning_numbers,
            playing_numbers,
        })
    }
}
impl Card {
    /// Get the card score
    ///
    /// Taking the count of overlapping numbers (how many of the playing numbers are winning) and
    /// return 2 to the power of that number - 1
    fn get_score(&self) -> u32 {
        let matches = self.get_match_count();
        if matches == 0 {
            return 0;
        }

        2u32.pow(matches as u32 - 1)
    }

    /// Get matches count
    ///
    /// Return the number of matching numbers between the playing and winning numbers
    fn get_match_count(&self) -> usize {
        self.playing_numbers
            .intersection(&self.winning_numbers)
            .count()
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32, AdventError> {
    let cards: Vec<Card> = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let score = cards.iter().map(Card::get_score).sum::<u32>();
    Ok(score)
}

fn part2(input: &str) -> Result<u32, AdventError> {
    let mut cards: Vec<Card> = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let mut counts = vec![1; cards.len()];

    for (idx, card) in cards.iter_mut().enumerate() {
        let count = counts[idx];
        let matches = card.get_match_count();
        if matches == 0 {
            continue;
        }

        for c in counts.iter_mut().skip(idx + 1).take(matches) {
            *c += count;
        }
    }

    Ok(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 30);
    }

    #[test]
    fn test_card_from_str() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card: Card = input.parse().unwrap();

        assert_eq!(
            card.winning_numbers,
            vec![41, 48, 83, 86, 17].into_iter().collect::<HashSet<_>>()
        );
        assert_eq!(
            card.playing_numbers,
            vec![83, 86, 6, 31, 17, 9, 48, 53]
                .into_iter()
                .collect::<HashSet<_>>()
        );
    }
}
