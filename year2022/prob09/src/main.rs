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

fn count_spots_tail_touched(instructions: Vec<(GridDirection, usize)>) -> Result<usize> {
    let mut touched_spots: HashSet<Coordinate<i32>> = HashSet::from([(0, 0)]);

    let mut head: Coordinate<i32> = (0, 0);
    let mut tail: Coordinate<i32> = (0, 0);

    for (direction, steps) in instructions {
        for _ in 0..steps {
            match direction {
                GridDirection::Up => head.0 += 1,
                GridDirection::Down => head.0 -= 1,
                GridDirection::Left => head.1 -= 1,
                GridDirection::Right => head.1 += 1,
                _ => panic!("Bad direction"),
            }
            let (dx, dy) = get_movement(head, tail);
            tail.0 += dx;
            tail.1 += dy;
            touched_spots.insert(tail);
        }
    }

    Ok(touched_spots.len())
}

/// Calculate movement required to bring t to h
///
/// This should be called after moving h, to know what movement is required to be applied to t
fn get_movement(h: Coordinate<i32>, t: Coordinate<i32>) -> Coordinate<i32> {
    match (h.0 - t.0, h.1 - t.1) {
        // Same spot, no movement required
        (0, 0) => (0, 0),
        // Next to each other, no movement required
        (1, 0) | (0, 1) | (-1, 0) | (0, -1) => (0, 0),
        // Diagonal, no movement required
        (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
        // 2 steps away, the tail should follow directly
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        // Moved away from a diagonal position
        (2, 1) => (1, 1),
        (2, -1) => (1, -1),
        (-2, 1) => (-1, 1),
        (-2, -1) => (-1, -1),
        (1, 2) => (1, 1),
        (1, -2) => (1, -1),
        (-1, 2) => (-1, 1),
        (-1, -2) => (-1, -1),
        // Far diagonal?
        (2, 2) => (1, 1),
        (2, -2) => (1, -1),
        (-2, 2) => (-1, 1),
        (-2, -2) => (-1, -1),
        unknown => panic!("Unknown movement: {:?}", unknown),
    }
}

/// This is just like in part 1, except now we have a total of 9 parts to the tail (10 total parts
/// including the head)
///
/// The only difference is that when moving the tail, we move check if we need to move each
/// following part of the tail
/// The movement of each tail part is the same, until no more parts need to move.
///
/// We are counting how many spots the _last_ part visits
fn count_spots_with_long_tail_touched(instructions: Vec<(GridDirection, usize)>) -> Result<usize> {
    let mut touched_spots: HashSet<Coordinate<i32>> = HashSet::from([(0, 0)]);

    let mut head: Coordinate<i32> = (0, 0);
    let tail: &mut [Coordinate<i32>] = &mut [(0, 0); 9];

    for (direction, steps) in instructions {
        for _ in 0..steps {
            match direction {
                GridDirection::Up => head.0 += 1,
                GridDirection::Down => head.0 -= 1,
                GridDirection::Left => head.1 -= 1,
                GridDirection::Right => head.1 += 1,
                _ => panic!("Bad direcion"),
            }

            let (dx, dy) = get_movement(head, tail[0]);
            tail[0].0 += dx;
            tail[0].1 += dy;

            for i in 1..tail.len() {
                let (dx, dy) = get_movement(tail[i - 1], tail[i]);
                // If no movement, then we don't need to check the rest of the tail
                if dx == 0 && dy == 0 {
                    break;
                }
                tail[i].0 += dx;
                tail[i].1 += dy;
                if i == (tail.len() - 1) {
                    touched_spots.insert(tail[i]);
                }
            }
        }
    }

    Ok(touched_spots.len())
}

fn part1(input: &str) -> Result<usize> {
    let instructions = parse_instructions(input)?;
    count_spots_tail_touched(instructions)
}

fn part2(input: &str) -> Result<usize> {
    let instructions = parse_instructions(input)?;
    count_spots_with_long_tail_touched(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const LONG_TEST_INPUT: &str = include_str!("../test_long.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(LONG_TEST_INPUT).unwrap(), 36);
    }
}
