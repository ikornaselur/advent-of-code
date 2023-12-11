use advent_core::error::AdventError;
use advent_core::generic_error;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Node {
    galaxy: bool,
    distance: Option<usize>,
}

impl Node {
    fn new(galaxy: bool) -> Self {
        Node {
            galaxy,
            distance: None,
        }
    }

    fn visited(&self) -> bool {
        self.distance.is_some()
    }
}

#[derive(Debug, Default)]
struct DistanceMap {
    map: HashMap<(Coord, Coord), usize>,
}

impl DistanceMap {
    fn insert(&mut self, from: Coord, to: Coord, distance: usize) {
        if from > to {
            self.map.insert((to, from), distance);
        } else {
            self.map.insert((from, to), distance);
        }
    }

    fn get(&self, from: Coord, to: Coord) -> Option<&usize> {
        if from > to {
            self.map.get(&(to, from))
        } else {
            self.map.get(&(from, to))
        }
    }
}

/// An image is a list of strings, which contain "." for empty space and "#" for a galaxy
struct Image {
    map: Vec<Vec<bool>>, // true is a galaxy
}

impl Image {
    /// Expand the image
    ///
    /// Each row and column that has no galaxies in it needs to be doubled up
    ///
    /// This means that the map before of
    ///
    /// ..#
    /// #..
    /// ...
    ///
    /// will expand to
    ///
    /// ...#
    /// #...
    /// ....
    /// ....
    fn expand(&mut self) {
        // First iterare through the columns to expand
        let mut col = 0;
        while col < self.map[0].len() {
            if self.map.iter().all(|row| !row[col]) {
                // Add a new column
                for row in &mut self.map {
                    row.insert(col, false);
                }
                col += 1; // We skip the next column, since we just added it
            }
            col += 1;
        }

        // Then we iterate through the rows as well
        let mut row = 0;
        while row < self.map.len() {
            if self.map[row].iter().all(|&c| !c) {
                // Add a new row
                self.map.insert(row, vec![false; self.map[0].len()]);
                row += 1; // We skip the next row, since we just added it
            }
            row += 1
        }
    }

    /// Get the distance map for this image
    ///
    /// The distance map is the shortest distance between all galaxies
    fn get_distance_map(&self) -> Result<DistanceMap, AdventError> {
        let mut distance_map = DistanceMap::default();

        // Find all the galaxies
        let mut galaxy_coords: Vec<Coord> = vec![];
        for (y, row) in self.map.iter().enumerate() {
            for (x, &galaxy) in row.iter().enumerate() {
                if galaxy {
                    galaxy_coords.push((x, y));
                }
            }
        }

        // Then we start from each galaxy and search outwards until we reach all the others
        for galaxy_coord in galaxy_coords {
            // Create a node map to visit
            let mut node_map: Vec<Vec<Node>> = self
                .map
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&galaxy| Node::new(galaxy))
                        .collect::<Vec<Node>>()
                })
                .collect();
            node_map[galaxy_coord.1][galaxy_coord.0].distance = Some(0);

            let mut to_visit: VecDeque<Coord> = VecDeque::from(vec![galaxy_coord]);

            // Then we traverse the map
            while let Some(coord) = to_visit.pop_front() {
                let distance = node_map[coord.1][coord.0]
                    .distance
                    .ok_or(generic_error!("No distance for {:?}", coord))?;
                // Check the 4 directions
                for (x, y) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let new_coord = (coord.0 as isize + x, coord.1 as isize + y);
                    if new_coord.0 < 0
                        || new_coord.1 < 0
                        || new_coord.0 >= node_map[0].len() as isize
                        || new_coord.1 >= node_map.len() as isize
                    {
                        // Out of bounds
                        continue;
                    }
                    let new_coord = (new_coord.0 as usize, new_coord.1 as usize);
                    if node_map[new_coord.1][new_coord.0].visited() {
                        // Already visited
                        continue;
                    }
                    if self.map[new_coord.1][new_coord.0] {
                        // Galaxy
                        node_map[new_coord.1][new_coord.0].distance = Some(distance + 1);
                        to_visit.push_back(new_coord);
                        distance_map.insert(galaxy_coord, new_coord, distance + 1);
                    } else {
                        // Empty space
                        node_map[new_coord.1][new_coord.0].distance = Some(distance + 1);
                        to_visit.push_back(new_coord);
                    }
                }
            }
        }

        Ok(distance_map)
    }
}

impl FromStr for Image {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        Ok(Image { map })
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, AdventError> {
    let mut image: Image = input.parse()?;

    image.expand();

    let distance_map = image.get_distance_map()?;

    Ok(distance_map.map.values().sum())
}

fn part2(input: &str) -> Result<u32, AdventError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_image_from_str() {
        let image: Image = PART_1_TEST_INPUT.parse().unwrap();

        assert_eq!(image.map.len(), 10);
        assert_eq!(
            image.map[0],
            vec![false, false, false, true, false, false, false, false, false, false]
        );
    }

    #[test]
    fn test_image_expand_base_test() {
        let mut image: Image = "...\n#..\n..#".parse().unwrap();

        assert_eq!(image.map.len(), 3);
        assert_eq!(image.map[0].len(), 3);

        image.expand();

        assert_eq!(image.map.len(), 4);
        assert_eq!(image.map[0].len(), 4);
    }

    #[test]
    fn test_image_expand_bigger_input() {
        let mut image: Image = PART_1_TEST_INPUT.parse().unwrap();

        assert_eq!(image.map.len(), 10);
        assert_eq!(image.map[0].len(), 10);

        image.expand();

        assert_eq!(image.map.len(), 12);
        assert_eq!(image.map[0].len(), 13);
    }

    #[test]
    fn test_distance_map_inserts_by_sorted_tuple() {
        let mut distance_map = DistanceMap::default();

        distance_map.insert((0, 0), (1, 0), 1);
        distance_map.insert((1, 0), (0, 0), 1);

        assert_eq!(distance_map.map.len(), 1);
    }

    #[test]
    fn test_distance_map_get_by_sorted_tuple() {
        let mut distance_map = DistanceMap::default();

        distance_map.insert((0, 0), (1, 0), 1);

        assert_eq!(distance_map.get((0, 0), (1, 0)), Some(&1));
        assert_eq!(distance_map.get((1, 0), (0, 0)), Some(&1));
    }

    #[test]
    fn test_image_get_distance_map() {
        let image: Image = "....\n....\n##.#\n...#".parse().unwrap();

        // Map will be
        // ....
        // ....
        // ##.#
        // ...#

        let distance_map = image.get_distance_map().unwrap();

        assert_eq!(distance_map.map.len(), 6);
        assert_eq!(distance_map.get((0, 2), (3, 2)), Some(&3));
        assert_eq!(distance_map.get((0, 2), (3, 3)), Some(&4));
        assert_eq!(distance_map.get((3, 2), (3, 3)), Some(&1));
        assert_eq!(distance_map.get((1, 2), (0, 2)), Some(&1));
        assert_eq!(distance_map.get((1, 2), (3, 2)), Some(&2));
        assert_eq!(distance_map.get((1, 2), (3, 3)), Some(&3));
    }
}
