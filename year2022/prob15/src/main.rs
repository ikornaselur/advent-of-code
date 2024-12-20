use advent::prelude::*;
use parse::parse_input;
use rayon::prelude::*;
use std::ops::RangeInclusive;

mod parse;

#[derive(Debug, PartialEq, Eq)]
struct Sensor(GridCoordinate<i32>);

#[derive(Debug, PartialEq, Eq)]
struct Beacon(GridCoordinate<i32>);

fn main() -> Result<()> {
    let input = get_input(2022, 15)?;

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

fn get_intersections_to_row(
    input: &Vec<(Sensor, Beacon)>,
    check_row: i32,
) -> Vec<RangeInclusive<i32>> {
    // Note that the 'row' in question is for the 'x' coordinate in x,y
    // So the row is (check_row, y) where y is any value
    let mut intersections = Vec::new();

    for (Sensor(s_coord), Beacon(b_coord)) in input {
        // TODO: This can be cached
        let dist = s_coord.manhattan_distance(b_coord);
        let dist_to_check_row = (check_row - s_coord.row).abs();

        // Calculate if we intersect the check_row
        // There are three cases:
        // 1. No intersection
        // 2. Intersection in a single point (exactly the dist away)
        // 3. Intersection in a range of points (less than dist away)
        match dist - dist_to_check_row {
            // 2. Intersection in a single point
            0 => intersections.push(s_coord.column..=s_coord.column),
            // 3. Intersection in a range of points
            diff if diff > 0 => {
                intersections.push((s_coord.column - diff)..=(s_coord.column + diff))
            }
            // 1. No intersection
            _ => {}
        }
    }

    // Let's sort the ranges, so that we can combine the overlaps
    intersections.sort_by(|a, b| a.start().cmp(b.start()));

    intersections
}

fn count_intersections_to_row(input: Vec<(Sensor, Beacon)>, check_row: i32) -> i32 {
    let intersections = get_intersections_to_row(&input, check_row);

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

fn search_open_spot(min_coord: i32, max_coord: i32, input: Vec<(Sensor, Beacon)>) -> i64 {
    (min_coord..=max_coord)
        .into_par_iter()
        .find_map_first(|row| {
            let intersections = get_intersections_to_row(&input, row);
            let current_end = (*intersections[0].end()).min(max_coord);

            intersections
                .iter()
                .scan(current_end, |current_end, range| {
                    if *range.start() <= *current_end {
                        if *range.end() > *current_end {
                            *current_end = (*range.end()).min(max_coord);
                        }
                        Some(None)
                    } else {
                        // We've found a gap!
                        let col = *current_end + 1;
                        Some(Some((col as i64) * 4_000_000 + (row as i64)))
                    }
                })
                .find_map(|x| x)
        })
        .unwrap()
}

fn part1(input: &str) -> Result<i32> {
    let parsed_input = parse_input(input)?;
    Ok(count_intersections_to_row(parsed_input, 2_000_000))
}

fn part2(input: &str) -> Result<i64> {
    let parsed_input = parse_input(input)?;
    // NOTE: There's heaps of optimisations to be done here.. but hey, it runs in couple of
    // seconds in release mode, good enough for me!
    Ok(search_open_spot(0, 4_000_000, parsed_input))
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
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(search_open_spot(0, 20, parsed_input), 56_000_011);
    }
}
