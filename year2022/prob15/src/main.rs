use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Sensor(Coordinate<i32>);

#[derive(Debug, PartialEq, Eq)]
struct Beacon(Coordinate<i32>);

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn count_intersections_to_row(input: Vec<(Sensor, Beacon)>, check_row: i32) -> i32 {
    // Note that the 'row' in question is for the 'x' coordinate in x,y
    // So the row is (check_row, y) where y is any value
    let mut intersections = Vec::new();

    for (Sensor((s_row, s_col)), Beacon((b_row, b_col))) in input {
        let dist = manhattan_distance((s_row, s_col), (b_row, b_col));
        let dist_to_check_row = (check_row - s_row).abs();

        // Calculate if we intersect the check_row
        // There are three cases:
        // 1. No intersection
        // 2. Intersection in a single point (exactly the dist away)
        // 3. Intersection in a range of points (less than dist away)
        match dist - dist_to_check_row {
            // 2. Intersection in a single point
            0 => intersections.push(s_col..=s_col),
            // 3. Intersection in a range of points
            diff if diff > 0 => intersections.push((s_col - diff)..=(s_col + diff)),
            // 1. No intersection
            _ => {}
        }
    }

    // Let's sort the ranges, then combine the overlaps
    intersections.sort_by(|a, b| a.start().cmp(b.start()));

    let mut total_length = 0;
    let mut current_range = intersections[0].clone();
    for range in &intersections[1..] {
        if range.start() <= current_range.end() {
            if range.end() > current_range.end() {
                current_range = *current_range.start()..=*range.end();
            }
        } else {
            total_length += current_range.end() - current_range.start();
            current_range = range.clone();
        }
    }

    total_length += current_range.end() - current_range.start();

    total_length
}

fn part1(input: &str) -> Result<i32> {
    let parsed_input = parse_input(input)?;
    Ok(count_intersections_to_row(parsed_input, 2_000_000))
}

fn part2(input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(count_intersections_to_row(parsed_input, 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
