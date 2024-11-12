use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

const WIN_POINTS: u32 = 6;
const DRAW_POINTS: u32 = 3;

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from_str(s: &str) -> Option<Hand> {
        match s {
            "A" | "X" => Some(Hand::Rock),
            "B" | "Y" => Some(Hand::Paper),
            "C" | "Z" => Some(Hand::Scissors),
            _ => None,
        }
    }

    fn to_score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beats(&self, other: &Hand) -> bool {
        matches!((self, other), (Hand::Paper, Hand::Rock) | (Hand::Rock, Hand::Scissors) | (Hand::Scissors, Hand::Paper))
    }
}


fn part1(input: &str) -> Result<u32> {
    Ok(input.lines().fold(0, |acc, line| {
        // Parse each line into two hands variables
        let mut hands = line.split_whitespace().filter_map(Hand::from_str);
        if let (Some(left), Some(right)) = (hands.next(), hands.next()) {
            match (&left, &right) {
                (l, r) if r.beats(l) => acc + right.to_score() + WIN_POINTS, 
                (l, r) if r == l => acc + right.to_score() + DRAW_POINTS,
                _ => acc + right.to_score(),
            }
        } else {
            panic!("Invalid input");
        }
    }))
}

fn part2(input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
