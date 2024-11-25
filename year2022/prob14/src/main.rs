use advent::prelude::*;
use parse::parse_coordinate_lists;
use std::{thread, time};

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Sand,
    Stone,
    Air,
    Source,
}

impl Cell {
    fn is_air(&self) -> bool {
        matches!(self, Cell::Air | Cell::Source)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Cell::Source => '+',
            Cell::Sand => 'O',
            Cell::Stone => '#',
            Cell::Air => '.',
        };
        write!(f, "{}", c)
    }
}

struct Map {
    map: Vec<Vec<Cell>>,
    // Used to calculate how much to shift the x coordinates
    source: Coordinate<usize>,
}

/// A map based on coordinates
///
/// The coordinates are (x, y) with x being the row, growing down (1 is below 0) and y growing to
/// the right
impl Map {
    fn from_coordinates_lists(coordinates_lists: Vec<Vec<Coordinate<usize>>>) -> Self {
        // Find the edges of the coordinates. We know that sand will start pouring in at 500,0, so
        // we can start with the edges there
        let mut left_edge = 500;
        let mut right_edge = 500;
        let mut bottom_edge = 0;

        for coordinates in coordinates_lists.iter() {
            for (x, y) in coordinates {
                if *x < left_edge {
                    left_edge = *x;
                }
                if *x > right_edge {
                    right_edge = *x;
                }
                if *y > bottom_edge {
                    bottom_edge = *y;
                }
            }
        }
        let width = right_edge - left_edge;
        let mut map = vec![vec![Cell::Air; width + 1]; bottom_edge + 1];

        // Fill in the walls
        for coordinates in coordinates_lists {
            let points = coordinates.len();
            for i in 0..(points - 1) {
                match (coordinates[i], coordinates[i + 1]) {
                    ((x1, y1), (x2, y2)) if x1 == x2 && y1 < y2 =>
                    {
                        #[allow(clippy::needless_range_loop)]
                        for y in y1..=y2 {
                            map[y][x1 - left_edge] = Cell::Stone;
                        }
                    }
                    ((x1, y1), (x2, y2)) if x1 == x2 && y1 > y2 =>
                    {
                        #[allow(clippy::needless_range_loop)]
                        for y in y2..=y1 {
                            map[y][x1 - left_edge] = Cell::Stone;
                        }
                    }
                    ((x1, y1), (x2, y2)) if y1 == y2 && x1 < x2 => {
                        for x in x1..=x2 {
                            map[y1][x - left_edge] = Cell::Stone;
                        }
                    }
                    ((x1, y1), (x2, y2)) if y1 == y2 && x1 > x2 => {
                        for x in x2..=x1 {
                            map[y1][x - left_edge] = Cell::Stone;
                        }
                    }
                    (_, _) => panic!("Invalid coordinates"),
                }
            }
        }

        // And mark the source
        map[0][500 - left_edge] = Cell::Source;

        Map {
            map,
            source: (500 - left_edge, 0),
        }
    }

    fn spawn_sand(&mut self, animate: bool) -> bool {
        let (mut x, mut y) = self.source;

        // We now move the sand until it comes to a rest
        // Note: There are optimisations to be made, but for now we just move the sand one step at
        // a time
        loop {
            if animate {
                thread::sleep(time::Duration::from_millis(100));
                let prv = self.map[y][x];
                self.map[y][x] = Cell::Sand;
                print!("{}[2J", 27 as char);
                for row in &self.map {
                    for cell in row {
                        print!("{}", cell);
                    }
                    println!();
                }
                self.map[y][x] = prv;
            }
            if y == self.map.len() - 1 {
                // We've reached the bottom of the map, so we can't add more sand!
                return true;
            } else if self.map[y + 1][x].is_air() {
                // We move down
                y += 1;
            } else if x == 0 {
                // We've reached the left edge, can't add more sand
                return true;
            } else if self.map[y + 1][x - 1].is_air() {
                // We move down and left
                x -= 1;
                y += 1;
            } else if x == self.map[0].len() - 1 {
                // We've reached the right edge, can't add more sand
                return true;
            } else if self.map[y + 1][x + 1].is_air() {
                // We move down and right
                x += 1;
                y += 1;
            } else {
                // We're stuck, so we have come to a rest
                self.map[y][x] = Cell::Sand;
                return false;
            }
        }
    }
}

fn part1(input: &str) -> Result<u32> {
    let coordinates_lists = parse_coordinate_lists(input)?;
    let mut map = Map::from_coordinates_lists(coordinates_lists);

    let mut grains = 0;

    while !map.spawn_sand(false) {
        grains += 1;
    }

    Ok(grains)
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_map_init() {
        let coordinates_lists = parse_coordinate_lists(TEST_INPUT).unwrap();
        let map = Map::from_coordinates_lists(coordinates_lists);

        assert_eq!(map.map.len(), 10);
        assert_eq!(map.map[0].len(), 10);

        assert_eq!(map.source, (6, 0));
    }
}
