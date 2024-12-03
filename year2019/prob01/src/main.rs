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

fn calc_fuel(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

/// We need to calculate how much fuel we need for the fuel, which will need more fuel by itself..
/// continue calculating until we reach 0 (or below)
fn calc_total_fuel(mass: usize) -> usize {
    let mut fuel_left = calc_fuel(mass);
    let mut total_fuel = fuel_left;
    while fuel_left > 0 {
        fuel_left = calc_fuel(fuel_left);
        total_fuel += fuel_left;
    }
    total_fuel
}

fn part1(input: &str) -> Result<usize> {
    let masses = parse_input(input)?;

    Ok(masses.iter().map(|&mass| calc_fuel(mass)).sum())
}

fn part2(input: &str) -> Result<usize> {
    let masses = parse_input(input)?;

    Ok(masses.iter().map(|&mass| calc_total_fuel(mass)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 34_241);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 51_316);
    }

    #[test]
    fn test_calc_fuel() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100_756), 33_583);
    }

    #[test]
    fn test_calc_fuel_for_fuel() {
        assert_eq!(calc_total_fuel(1969), 966);
        assert_eq!(calc_total_fuel(100_756), 50_346);
    }
}
