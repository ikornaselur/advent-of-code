use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, INPUT)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, INPUT)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        INPUT,
    );

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let bit_list = parse_input(input)?;

    let mut gamma_rate_bits = vec![];
    let mut epsilon_rate_bits = vec![];
    for idx in 0..bit_list[0].len() {
        let one_count = bit_list.iter().filter(|x| x[idx]).count();
        let zero_count = bit_list.len() - one_count;

        if one_count > zero_count {
            gamma_rate_bits.push(1);
            epsilon_rate_bits.push(0);
        } else {
            gamma_rate_bits.push(0);
            epsilon_rate_bits.push(1);
        }
    }

    let gamma_rate = gamma_rate_bits.iter().fold(0, |acc, x| acc * 2 + x);
    let epsilon_rate = epsilon_rate_bits.iter().fold(0, |acc, x| acc * 2 + x);

    Ok(gamma_rate * epsilon_rate)
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
