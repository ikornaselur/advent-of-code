use advent::prelude::*;
use parse::parse_input;

mod parse;

fn main() -> Result<()> {
    let input = get_input(2024, 14)?;

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

fn get_pos_after_iterations(
    width: i32,
    height: i32,
    iterations: i32,
    pos: GridCoordinate<i32>,
    vector: GridCoordinate<i32>,
) -> GridCoordinate<i32> {
    GridCoordinate {
        row: (pos.row + (vector.row * iterations)).rem_euclid(width),
        column: (pos.column + (vector.column * iterations)).rem_euclid(height),
    }
}

fn get_safety_rating(
    width: i32,
    height: i32,
    iterations: i32,
    states: &[(GridCoordinate<i32>, GridCoordinate<i32>)],
) -> usize {
    let positions: Vec<GridCoordinate<i32>> = states
        .iter()
        .map(|(pos, vector)| get_pos_after_iterations(width, height, iterations, *pos, *vector))
        .collect();

    // We know the width and height are odd values, we want the middle value
    let width_mid = width / 2;
    let height_mid = height / 2;

    // We should be able to fold this into a four-way tuple.. but hey, this should be fine - it's
    // not that many entries
    let upper_left_quadrant = positions
        .iter()
        .filter(|GridCoordinate { row, column }| column < &width_mid && row < &height_mid)
        .count();
    let upper_right_quadrant = positions
        .iter()
        .filter(|GridCoordinate { row, column }| column > &width_mid && row < &height_mid)
        .count();
    let lower_left_quadrant = positions
        .iter()
        .filter(|GridCoordinate { row, column }| column < &width_mid && row > &height_mid)
        .count();
    let lower_right_quadrant = positions
        .iter()
        .filter(|GridCoordinate { row, column }| column > &width_mid && row > &height_mid)
        .count();

    upper_left_quadrant * upper_right_quadrant * lower_left_quadrant * lower_right_quadrant
}

fn get_iterations_for_unique_pos(
    width: i32,
    height: i32,
    states: &[(GridCoordinate<i32>, GridCoordinate<i32>)],
) -> usize {
    let mut iterations = 0;
    let mut states = states.to_vec();

    loop {
        iterations += 1;
        states = states
            .iter()
            .map(|(pos, vector)| {
                (
                    get_pos_after_iterations(width, height, 1, *pos, *vector),
                    *vector,
                )
            })
            .collect();
        // Check if all positions are unique?
        let unique =
            HashSet::<GridCoordinate<i32>>::from_iter(states.iter().map(|(pos, _)| *pos)).len();
        if unique == states.len() {
            break;
        }
    }

    iterations
}

fn part1(input: &str) -> Result<usize> {
    let states = parse_input(input)?;
    Ok(get_safety_rating(101, 103, 100, &states))
}

fn part2(input: &str) -> Result<usize> {
    let states = parse_input(input)?;
    Ok(get_iterations_for_unique_pos(101, 103, &states))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let states = parse_input(TEST_INPUT).unwrap();
        let safety_rating = get_safety_rating(11, 7, 100, &states);
        assert_eq!(safety_rating, 12);
    }
}
