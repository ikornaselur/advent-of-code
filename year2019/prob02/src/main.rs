use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn run_program(digits: &mut [usize]) {
    let mut idx = 0;

    while let Some(val) = &digits.get(idx) {
        match val {
            1 => {
                let a = *digits.get(idx + 1).unwrap();
                let b = *digits.get(idx + 2).unwrap();
                let dest = *digits.get(idx + 3).unwrap();
                digits[dest] = digits[a] + digits[b];
                idx += 4;
            }
            2 => {
                let a = *digits.get(idx + 1).unwrap();
                let b = *digits.get(idx + 2).unwrap();
                let dest = *digits.get(idx + 3).unwrap();
                digits[dest] = digits[a] * digits[b];
                idx += 4;
            }
            99 => {
                break;
            }
            _ => panic!("Unknown opcode"),
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut digits = parse_input(input)?;

    digits[1] = 12;
    digits[2] = 2;

    run_program(&mut digits);

    Ok(digits[0])
}

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        // The test example doesn't involve changing the early digits.. so we skip that
        let mut digits = parse_input(TEST_INPUT).unwrap();
        run_program(&mut digits);

        assert_eq!(digits[0], 3500);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
