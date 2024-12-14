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

fn get_pos_after_iterations(
    width: i32,
    height: i32,
    iterations: i32,
    pos: Coordinate<i32>,
    vector: Coordinate<i32>,
) -> Coordinate<i32> {
    (
        (pos.0 + (vector.0 * iterations)).rem_euclid(width),
        (pos.1 + (vector.1 * iterations)).rem_euclid(height),
    )
}

fn get_safety_rating(
    width: i32,
    height: i32,
    iterations: i32,
    states: &[(Coordinate<i32>, Coordinate<i32>)],
) -> usize {
    let positions: Vec<Coordinate<i32>> = states
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
        .filter(|(x, y)| x < &width_mid && y < &height_mid)
        .count();
    let upper_right_quadrant = positions
        .iter()
        .filter(|(x, y)| x > &width_mid && y < &height_mid)
        .count();
    let lower_left_quadrant = positions
        .iter()
        .filter(|(x, y)| x < &width_mid && y > &height_mid)
        .count();
    let lower_right_quadrant = positions
        .iter()
        .filter(|(x, y)| x > &width_mid && y > &height_mid)
        .count();

    upper_left_quadrant * upper_right_quadrant * lower_left_quadrant * lower_right_quadrant
}

fn get_iterations_for_unique_pos(
    width: i32,
    height: i32,
    states: &[(Coordinate<i32>, Coordinate<i32>)],
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
            HashSet::<Coordinate<i32>>::from_iter(states.iter().map(|(pos, _)| *pos)).len();
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
