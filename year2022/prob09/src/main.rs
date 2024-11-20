use advent::prelude::*;
use parse::parse_instructions;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

/// Calculate the manhattan distance and whether the coordinates are touching
///
/// The coordinates are touching if the manhattan distance is:
/// * 0 - On top of each other
/// * 1 - Right next to each other
/// * 2 - IF the distance is 1x1 away, so touching diagonally
fn is_touching(a: Coordinate<i32>, b: Coordinate<i32>) -> bool {
    let x_diff = (a.0 - b.0).abs();
    let y_diff = (a.1 - b.1).abs();

    x_diff <= 1 && y_diff <= 1
}

fn count_spots_tail_touched(instructions: Vec<(OrdinalDirection, usize)>) -> Result<usize> {
    let mut touched_spots: HashSet<Coordinate<i32>> = HashSet::from([(0, 0)]);

    let mut head: Coordinate<i32> = (0, 0);
    let mut tail: Coordinate<i32> = (0, 0);

    for (direction, steps) in instructions {
        for _ in 0..steps {
            let prev_head = head;

            match direction {
                OrdinalDirection::Up => head.1 += 1,
                OrdinalDirection::Down => head.1 -= 1,
                OrdinalDirection::Left => head.0 -= 1,
                OrdinalDirection::Right => head.0 += 1,
            }

            if !is_touching(head, tail) {
                tail = prev_head;
                touched_spots.insert(tail);
            }
        }
    }

    Ok(touched_spots.len())
}

fn part1(input: &str) -> Result<usize> {
    let instructions = parse_instructions(input)?;
    count_spots_tail_touched(instructions)
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
