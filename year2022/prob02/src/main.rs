use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Goal {
    Win,
    Draw,
    Lose,
}

impl Goal {
    fn from_str(s: &str) -> Option<Goal> {
        match s {
            "X" => Some(Goal::Lose),
            "Y" => Some(Goal::Draw),
            "Z" => Some(Goal::Win),
            _ => None,
        }
    }

    fn to_score(&self) -> u32 {
        match self {
            Goal::Win => 6,
            Goal::Draw => 3,
            Goal::Lose => 0,
        }
    }
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

    fn better(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn worse(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
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
                (l, r) if r.beats(l) => acc + right.to_score() + Goal::Win.to_score(), 
                (l, r) if r == l => acc + right.to_score() + Goal::Draw.to_score(),
                _ => acc + right.to_score(),
            }
        } else {
            panic!("Invalid input");
        }
    }))
}

fn part2(input: &str) -> Result<u32> {
    Ok(input.lines().fold(0, |acc, line| {
        let mut chars = line.split_whitespace();

        if let (Some(left), Some(right)) = (chars.next(), chars.next()) {
            let hand = Hand::from_str(left).unwrap();
            let goal = Goal::from_str(right).unwrap();

            let opposite_hand = match &goal {
                Goal::Draw => hand,
                Goal::Win => hand.better(),
                Goal::Lose => hand.worse(),
            };

            acc + opposite_hand.to_score() + goal.to_score()
            
        } else {
            panic!("Invalid input");
        }
    }))
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 12);
    }
}
