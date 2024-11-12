use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq)]
struct Game {
    num: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq)]
struct Round {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl FromStr for Game {
    type Err = AdventError;

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = line.split(": ");
        let num = parts
            .next()
            .ok_or(parse_error!("Unable to parse parts from line"))?
            .trim_start_matches("Game ")
            .parse::<u32>()?;

        let rounds_str_parts = parts
            .next()
            .ok_or(parse_error!("Unable to parse rounds from string"))?
            .split("; ");
        let mut rounds = Vec::new();
        for round_str in rounds_str_parts {
            let mut round = Round {
                red: None,
                green: None,
                blue: None,
            };
            let colours = round_str.split(", ");
            for colour in colours {
                let mut parts = colour.split(' ');
                let amount = parts
                    .next()
                    .ok_or(parse_error!("Unable to parse amount"))?
                    .parse::<u32>()?;
                let colour = parts.next().ok_or(parse_error!("Unable to parse colour"))?;
                match colour {
                    "red" => round.red = Some(amount),
                    "green" => round.green = Some(amount),
                    "blue" => round.blue = Some(amount),
                    _ => panic!("Unknown colour"),
                }
            }
            rounds.push(round);
        }

        Ok(Game { num, rounds })
    }
}

impl Game {
    fn above_max(&self, red: u32, green: u32, blue: u32) -> bool {
        for round in &self.rounds {
            if let Some(amount) = round.red {
                if amount > red {
                    return true;
                }
            }
            if let Some(amount) = round.green {
                if amount > green {
                    return true;
                }
            }
            if let Some(amount) = round.blue {
                if amount > blue {
                    return true;
                }
            }
        }
        false
    }

    fn min_power(&self) -> u32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for round in &self.rounds {
            if let Some(amount) = round.red {
                min_red = std::cmp::max(min_red, amount);
            }
            if let Some(amount) = round.green {
                min_green = std::cmp::max(min_green, amount);
            }
            if let Some(amount) = round.blue {
                min_blue = std::cmp::max(min_blue, amount);
            }
        }
        min_red * min_green * min_blue
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
    let red = 12;
    let green = 13;
    let blue = 14;

    input
        .lines()
        .map(str::parse::<Game>)
        .try_fold(0, |acc, game| {
            let game = game?;
            if game.above_max(red, green, blue) {
                Ok(acc)
            } else {
                Ok(acc + game.num)
            }
        })
}

fn part2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(str::parse::<Game>)
        .try_fold(0, |acc, game| Ok(acc + game?.min_power()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 2286);
    }

    #[test]
    fn test_game_from_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game: Game = line.parse().unwrap();

        assert_eq!(
            game,
            Game {
                num: 1,
                rounds: vec![
                    Round {
                        red: Some(4),
                        green: None,
                        blue: Some(3),
                    },
                    Round {
                        red: Some(1),
                        green: Some(2),
                        blue: Some(6),
                    },
                    Round {
                        red: None,
                        green: Some(2),
                        blue: None,
                    },
                ],
            }
        );
    }
}
