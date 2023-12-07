use advent_core::error::AdventError;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<(i64, i64)>, AdventError> {
    let mut lines = input.lines();

    let times = lines
        .next()
        .ok_or(AdventError::InvalidInput)?
        .strip_prefix("Time:")
        .ok_or(AdventError::InvalidInput)?
        .split_whitespace()
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let distances = lines
        .next()
        .ok_or(AdventError::InvalidInput)?
        .strip_prefix("Distance:")
        .ok_or(AdventError::InvalidInput)?
        .split_whitespace()
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // Return the zipped pairs
    Ok(times.into_iter().zip(distances).collect())
}

/// Solve a hyperbola of the form:
/// x * (y - x) = d
///
/// Returns the two solutions for x, panic if two solutions not found
fn solve(y: i64, d: i64) -> (f64, f64) {
    let discriminant = y.pow(2) as f64 - 4.0 * (d as f64 + 0.01);
    let x1 = ((y as f64) + discriminant.sqrt()) / 2.0;
    let x2 = ((y as f64) - discriminant.sqrt()) / 2.0;

    (x1, x2)
}

fn part1(input: &str) -> Result<i64, AdventError> {
    let mut result = 1;

    for (time, distance) in parse_input(input)? {
        let (x1, x2) = match solve(time, distance) {
            (x1, x2) if x1 > x2 => (x1, x2),
            (x1, x2) => (x2, x1),
        };

        let x1 = x1.ceil() as i64;
        let x2 = x2.ceil() as i64;

        result *= x1 - x2;
    }

    Ok(result)
}

fn part2(input: &str) -> Result<i64, AdventError> {
    part1(&input.replace(' ', ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 71503);
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(input, vec![(7, 9), (15, 40), (30, 200)]);
    }
}
