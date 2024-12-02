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

fn part1(input: &str) -> Result<u32> {
    let reports = parse_input(input)?;
    let mut safe_reports = 0;
    for report in reports {
        // Check if the levels are either all increasing or all decreasing
        let diffs = report.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        let increasing = diffs.iter().all(|&x| x > 0);
        let decreasing = diffs.iter().all(|&x| x < 0);
        let max_diff = diffs.iter().map(|d| d.abs()).max().unwrap();

        if (increasing || decreasing) && max_diff <= 3 {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
