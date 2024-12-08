use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Empty,
    Antenna(char),
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let antenna_map = parse_input(input)?;
    let height = antenna_map.len();
    let width = antenna_map[0].len();

    let mut antenna_coordinates = HashMap::new();

    // Store a list of coordinates for each antenna type
    for (y, row) in antenna_map.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            if let Node::Antenna(id) = node {
                if !antenna_coordinates.contains_key(id) {
                    antenna_coordinates.insert(id, Vec::new());
                }
                antenna_coordinates
                    .get_mut(id)
                    .ok_or(error!("Unable to get antenna coordinates"))?
                    .push((y as isize, x as isize));
            }
        }
    }

    let mut antinodes = HashSet::new();
    // Calculate distance between all antennas of the same
    for (_antenna_id, coordinates) in antenna_coordinates.iter() {
        for (y1, x1) in coordinates.iter() {
            for (y2, x2) in coordinates.iter() {
                if (y1, x1) == (y2, x2) {
                    continue;
                }
                // Calculate the vector between them, doubled up, and add it to the coordinate.
                // This will be an 'antinode'.
                let antinode = (y1 + (y2 - y1) * 2, x1 + (x2 - x1) * 2);

                // Skip antinodes outside of the map
                if antinode.0 < 0
                    || antinode.0 >= height as isize
                    || antinode.1 < 0
                    || antinode.1 >= width as isize
                {
                    continue;
                }

                antinodes.insert(antinode);
            }
        }
    }

    Ok(antinodes.len())
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
