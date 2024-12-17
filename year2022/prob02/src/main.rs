use advent::prelude::*;

fn main() -> Result<()> {
    let input = get_input(2022, 2)?;

    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, &input)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, &input)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        &input,
    );

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Goal {
    Win,
    Draw,
    Lose,
}

impl FromStr for Goal {
    type Err = AdventError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "X" => Ok(Goal::Lose),
            "Y" => Ok(Goal::Draw),
            "Z" => Ok(Goal::Win),
            _ => Err(AdventError::InvalidInput),
        }
    }
}

impl Goal {
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

impl FromStr for Hand {
    type Err = AdventError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(AdventError::InvalidInput),
        }
    }
}

impl Hand {
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
        self.worse() == *other
    }
}

fn part1(input: &str) -> Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        // Parse each line into two hands variables
        let mut hands = line.split_whitespace().map(Hand::from_str);

        if let (Some(Ok(left)), Some(Ok(right))) = (hands.next(), hands.next()) {
            Ok(match (&left, &right) {
                (l, r) if r.beats(l) => acc + right.to_score() + Goal::Win.to_score(),
                (l, r) if r == l => acc + right.to_score() + Goal::Draw.to_score(),
                _ => acc + right.to_score(),
            })
        } else {
            panic!("Invalid input");
        }
    })
}

fn part2(input: &str) -> Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        let mut chars = line.split_whitespace();

        if let (Some(left), Some(right)) = (chars.next(), chars.next()) {
            let hand: Hand = left.parse()?;
            let goal: Goal = right.parse()?;

            let opposite_hand = match &goal {
                Goal::Draw => hand,
                Goal::Win => hand.better(),
                Goal::Lose => hand.worse(),
            };

            Ok(acc + opposite_hand.to_score() + goal.to_score())
        } else {
            panic!("Invalid input");
        }
    })
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
