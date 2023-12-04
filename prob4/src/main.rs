use advent_core::error::AdventError;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

struct Card {
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl Card {
    /// Create a new card from a string
    ///
    /// The string should be in this form:
    ///
    /// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    ///
    /// where the first 5 numbers left of the | are the winning numbers and on the right are the
    /// playing numbers
    fn from_str(input: &str) -> Result<Self, AdventError> {
        let input = input.split_whitespace().collect::<Vec<_>>().join(" ");
        // Get everything after the first :
        let input = input.split(": ").collect::<Vec<_>>()[1];
        let mut numbers = input.split(" | ");
        let winning_numbers = numbers
            .next()
            .ok_or(AdventError::InvalidInput)?
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let playing_numbers = numbers
            .next()
            .ok_or(AdventError::InvalidInput)?
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            winning_numbers,
            playing_numbers,
        })
    }

    /// Get the card score
    ///
    /// Taking the count of overlapping numbers (how many of the playing numbers are winning) and
    /// return 2 to the power of that number - 1
    fn get_score(&self) -> u32 {
        let playing_numbers: HashSet<_> = self.playing_numbers.iter().collect();
        let winning_numbers: HashSet<_> = self.winning_numbers.iter().collect();
        let overlapping_numbers = playing_numbers.intersection(&winning_numbers).count();

        if overlapping_numbers == 0 {
            return 0;
        }

        2u32.pow(overlapping_numbers as u32 - 1)
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
    let cards = input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let score = cards.iter().map(Card::get_score).sum::<u32>();
    Ok(score)
}

fn part2(input: &str) -> Result<u32, AdventError> {
    Ok(0)
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
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_card_from_str() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(input).unwrap();

        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17],);
        assert_eq!(card.playing_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53],);
    }
}
