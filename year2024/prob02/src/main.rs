use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");
const MAX_LEVEL: i32 = 3;

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

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

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }
    let mut prev_diff = report[1] - report[0];
    if prev_diff == 0 {
        return false; // Already not safe
    }

    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff.signum() != prev_diff.signum() || diff.abs() > MAX_LEVEL {
            return false;
        }
        prev_diff = diff;
    }
    true
}

fn part1(input: &str) -> Result<usize> {
    let reports = parse_input(input)?;
    let safe_reports = reports.iter().filter(|&r| is_safe(r)).count();
    Ok(safe_reports)
}

fn part2(input: &str) -> Result<usize> {
    let reports = parse_input(input)?;

    let safe_reports = reports
        .iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|i| {
                    let mut filtered_report = Vec::with_capacity(report.len() - 1);
                    filtered_report.extend_from_slice(&report[..i]);
                    filtered_report.extend_from_slice(&report[i + 1..]);
                    is_safe(&filtered_report)
                })
        })
        .count();
    Ok(safe_reports)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 4);
    }
}
