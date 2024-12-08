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

fn get_antenna_coordinates(
    antenna_map: &[Vec<Node>],
) -> Result<HashMap<&char, Vec<Coordinate<isize>>>> {
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

    Ok(antenna_coordinates)
}

fn get_antinodes(
    height: isize,
    width: isize,
    antenna_coordinates: HashMap<&char, Vec<Coordinate<isize>>>,
    with_harmonics: bool,
) -> HashSet<Coordinate<isize>> {
    let mut antinodes = HashSet::new();
    // Calculate distance between all antennas of the same
    for (_antenna_id, coordinates) in antenna_coordinates.iter() {
        for (y1, x1) in coordinates.iter() {
            for (y2, x2) in coordinates.iter() {
                if (y1, x1) == (y2, x2) {
                    continue;
                }
                let vector = (y2 - y1, x2 - x1);
                let mut start_node = if with_harmonics {
                    (*y1, *x1)
                } else {
                    (y1 + vector.0, x1 + vector.1)
                };
                loop {
                    let antinode = (start_node.0 + vector.0, start_node.1 + vector.1);

                    if antinode.0 < 0
                        || antinode.0 >= height
                        || antinode.1 < 0
                        || antinode.1 >= width
                    {
                        break;
                    }
                    antinodes.insert(antinode);
                    if !with_harmonics {
                        break;
                    }

                    start_node = antinode;
                }
            }
        }
    }
    antinodes
}

fn part1(input: &str) -> Result<usize> {
    let antenna_map = parse_input(input)?;
    let height = antenna_map.len();
    let width = antenna_map[0].len();

    let antenna_coordinates = get_antenna_coordinates(&antenna_map)?;
    let antinodes = get_antinodes(height as isize, width as isize, antenna_coordinates, false);

    Ok(antinodes.len())
}

fn part2(input: &str) -> Result<usize> {
    let antenna_map = parse_input(input)?;
    let height = antenna_map.len();
    let width = antenna_map[0].len();

    let antenna_coordinates = get_antenna_coordinates(&antenna_map)?;
    let antinodes = get_antinodes(height as isize, width as isize, antenna_coordinates, true);

    Ok(antinodes.len())
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 34);
    }
}
