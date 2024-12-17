use advent::prelude::*;
use parse::parse_input;

mod parse;

fn main() -> Result<()> {
    let input = get_input(2021, 3)?;

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

fn part2(input: &str) -> Result<usize> {
    let bit_list = parse_input(input)?;

    // Oxygen generator rating
    let mut oxygen_rate_bits = bit_list.clone();
    let mut idx = 0;
    while oxygen_rate_bits.len() > 1 {
        let one_count = oxygen_rate_bits.iter().filter(|x| x[idx]).count();
        let zero_count = oxygen_rate_bits.len() - one_count;

        if one_count >= zero_count {
            oxygen_rate_bits.retain(|x| x[idx]);
        } else {
            oxygen_rate_bits.retain(|x| !x[idx]);
        }

        idx += 1;
    }

    // CO2 scrubber rating
    let mut co2_rate_bits = bit_list.clone();
    let mut idx = 0;
    while co2_rate_bits.len() > 1 {
        let one_count = co2_rate_bits.iter().filter(|x| x[idx]).count();
        let zero_count = co2_rate_bits.len() - one_count;

        if one_count >= zero_count {
            co2_rate_bits.retain(|x| !x[idx]);
        } else {
            co2_rate_bits.retain(|x| x[idx]);
        }

        idx += 1;
    }

    // Turn the bits into values
    let oxygen_rate = oxygen_rate_bits[0]
        .iter()
        .map(|x| if *x { 1 } else { 0 })
        .fold(0, |acc, x| acc * 2 + x);
    let co2_rate = co2_rate_bits[0]
        .iter()
        .map(|x| if *x { 1 } else { 0 })
        .fold(0, |acc, x| acc * 2 + x);

    Ok(oxygen_rate * co2_rate)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 230);
    }
}
