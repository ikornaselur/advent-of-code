use advent::prelude::*;
use parse::parse_input;

mod parse;

fn main() -> Result<()> {
    let input = get_input(2024, 10)?;

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

/// Returns the difference between the chars
///
/// This is done by subtracting a from b, so that if b is larger than the diff is positive
fn get_node_diff(first: char, second: char) -> i8 {
    second as i8 - first as i8
}

fn get_path_trailheads(map: &[Vec<char>], pos: Coordinate<usize>) -> Vec<Coordinate<usize>> {
    let mut trailheads = Vec::new();
    let node = map[pos.0][pos.1];

    let height = map.len();
    let width = map[0].len();

    if node == '9' {
        trailheads.push(pos);
    } else {
        if pos.0 > 0 && get_node_diff(node, map[pos.0 - 1][pos.1]) == 1 {
            trailheads.extend(get_path_trailheads(map, (pos.0 - 1, pos.1)));
        }
        if pos.0 < height - 1 && get_node_diff(node, map[pos.0 + 1][pos.1]) == 1 {
            trailheads.extend(get_path_trailheads(map, (pos.0 + 1, pos.1)));
        }
        if pos.1 > 0 && get_node_diff(node, map[pos.0][pos.1 - 1]) == 1 {
            trailheads.extend(get_path_trailheads(map, (pos.0, pos.1 - 1)));
        }
        if pos.1 < width - 1 && get_node_diff(node, map[pos.0][pos.1 + 1]) == 1 {
            trailheads.extend(get_path_trailheads(map, (pos.0, pos.1 + 1)));
        }
    }
    trailheads
}

fn get_path_score(map: &[Vec<char>], pos: Coordinate<usize>, unique_paths: bool) -> usize {
    match map[pos.0][pos.1] {
        '0' => {
            if unique_paths {
                get_path_trailheads(map, pos).len()
            } else {
                HashSet::<_>::from_iter(get_path_trailheads(map, pos)).len()
            }
        }
        _ => 0,
    }
}

fn part1(input: &str) -> Result<usize> {
    let map = parse_input(input)?;
    Ok(map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (x, _)| acc + get_path_score(&map, (y, x), false))
    }))
}

fn part2(input: &str) -> Result<usize> {
    let map = parse_input(input)?;
    Ok(map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (x, _)| acc + get_path_score(&map, (y, x), true))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 81);
    }
}
