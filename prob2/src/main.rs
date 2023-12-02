const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("## Part 1");
    println!(" > {}", part1(INPUT));

    println!("## Part 2");
    println!(" > {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let red = 12;
    let green = 13;
    let blue = 14;

    let mut result = 0;

    'outer: for line in input.lines() {
        let mut parts = line.split(':');

        let game_number = parts
            .next()
            .unwrap()
            .trim_start_matches("Game ")
            .parse::<u32>()
            .unwrap();

        let rounds = parts.next().unwrap().split(';');

        for round in rounds {
            for colour in round.split(',') {
                let mut parts = colour.trim().split(' ');
                let amount = parts.next().unwrap().parse::<u32>().unwrap();
                let colour = parts.next().unwrap();
                match colour {
                    "red" => {
                        if amount > red {
                            continue 'outer;
                        }
                    }
                    "green" => {
                        if amount > green {
                            continue 'outer;
                        }
                    }
                    "blue" => {
                        if amount > blue {
                            continue 'outer;
                        }
                    }
                    _ => panic!("Unknown colour"),
                }
            }
        }

        result += game_number;
    }

    result
}

fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT), 0);
    }
}
