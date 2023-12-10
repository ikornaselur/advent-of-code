use advent_core::error::AdventError;
use advent_core::generic_error;
use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

type Coordinate = (usize, usize);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Pipe {
    Horizontal,
    Vertical,
    CornerNorthEast,
    CornerNorthWest,
    CornerSouthEast,
    CornerSouthWest,
    Start,
    None,
}

impl FromStr for Pipe {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Horizontal),
            "|" => Ok(Self::Vertical),
            "L" => Ok(Self::CornerNorthEast),
            "J" => Ok(Self::CornerNorthWest),
            "F" => Ok(Self::CornerSouthEast),
            "7" => Ok(Self::CornerSouthWest),
            "S" => Ok(Self::Start),
            "." => Ok(Self::None),
            _ => Err(generic_error!("Invalid pipe: {}", s)),
        }
    }
}

impl Pipe {
    fn connects_to(&self) -> HashSet<Direction> {
        match self {
            Pipe::Horizontal => vec![Direction::East, Direction::West],
            Pipe::Vertical => vec![Direction::North, Direction::South],
            Pipe::CornerNorthEast => vec![Direction::North, Direction::East],
            Pipe::CornerNorthWest => vec![Direction::North, Direction::West],
            Pipe::CornerSouthEast => vec![Direction::South, Direction::East],
            Pipe::CornerSouthWest => vec![Direction::South, Direction::West],
            _ => vec![],
        }
        .into_iter()
        .collect()
    }
}

// Coordinates are (row, col)
struct PipeMap {
    nodes: Vec<Vec<Pipe>>,
    height: usize,
    width: usize,
}

impl FromStr for PipeMap {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<Pipe>())
                    .collect::<Result<Vec<Pipe>, Self::Err>>()
            })
            .collect::<Result<Vec<Vec<Pipe>>, Self::Err>>()?;

        let height = nodes.len();
        let width = nodes[0].len();
        Ok(Self {
            nodes,
            height,
            width,
        })
    }
}

impl PipeMap {
    /// Find the start node and return the coordinate
    ///
    /// The start node is the 'S' node
    fn find_start(&self) -> Result<Coordinate, AdventError> {
        for (x, row) in self.nodes.iter().enumerate() {
            for (y, node) in row.iter().enumerate() {
                if *node == Pipe::Start {
                    return Ok((x, y));
                }
            }
        }
        Err(generic_error!("Start node not found"))
    }

    /// Get the node at the given coordinate
    fn get_node(&self, coord: Coordinate) -> Result<&Pipe, AdventError> {
        let (row, col) = coord;
        if row >= self.height || col >= self.width {
            return Err(generic_error!("Invalid coordinate: {:?}", coord));
        }
        Ok(&self.nodes[row][col])
    }

    /// Look at the nodes around to see which node is the next one
    fn get_next_node(
        &self,
        current_node_coord: Coordinate,
        came_from_direction: Direction,
    ) -> Result<(Coordinate, Direction), AdventError> {
        let current_node = self.get_node(current_node_coord)?;
        if *current_node == Pipe::None || *current_node == Pipe::Start {
            return Err(generic_error!("Invalid node: {:?}", current_node));
        }

        // Get the next direction for this pipe
        let binding = &vec![came_from_direction].into_iter().collect();
        let connects_to = current_node.connects_to();
        let next_direction = connects_to
            .difference(binding)
            .next()
            .ok_or_else(|| generic_error!("No next direction found for {:?}", current_node))?;

        // Check if that next direction is valid
        let next_node_coord = match next_direction {
            Direction::North => (current_node_coord.0 - 1, current_node_coord.1),
            Direction::South => (current_node_coord.0 + 1, current_node_coord.1),
            Direction::West => (current_node_coord.0, current_node_coord.1 - 1),
            Direction::East => (current_node_coord.0, current_node_coord.1 + 1),
        };

        if next_node_coord.0 >= self.height || next_node_coord.1 >= self.width {
            return Err(generic_error!(
                "Invalid next node coordinate: {:?}",
                next_node_coord
            ));
        }

        let next_node = self.get_node(next_node_coord)?;
        if *next_node == Pipe::None {
            return Err(generic_error!("Invalid next node: {:?}", next_node));
        }

        Ok((next_node_coord, next_direction.clone().opposite()))
    }

    // Get the two directions that connect to the start
    fn get_start_directions(&self) -> Result<Vec<Direction>, AdventError> {
        let start_coord = self.find_start()?;
        let mut directions = vec![];

        // Check the four nodes around to see if any of them connect back

        // North node
        if let Some(north_coord) = self.shift_coord(start_coord, Direction::North) {
            let north_node = self.get_node(north_coord)?;
            if north_node.connects_to().contains(&Direction::South) {
                directions.push(Direction::North);
            }
        }

        // South node
        if let Some(south_coord) = self.shift_coord(start_coord, Direction::South) {
            let south_node = self.get_node(south_coord)?;
            if south_node.connects_to().contains(&Direction::North) {
                directions.push(Direction::South);
            }
        }

        // West node
        if let Some(west_coord) = self.shift_coord(start_coord, Direction::West) {
            let west_node = self.get_node(west_coord)?;
            if west_node.connects_to().contains(&Direction::East) {
                directions.push(Direction::West);
            }
        }

        // East node
        if let Some(east_coord) = self.shift_coord(start_coord, Direction::East) {
            let east_node = self.get_node(east_coord)?;
            if east_node.connects_to().contains(&Direction::West) {
                directions.push(Direction::East);
            }
        }

        if directions.len() != 2 {
            return Err(generic_error!(
                "Invalid number of directions found: {:?} - expected 2!",
                directions
            ));
        }

        Ok(directions)
    }

    fn shift_coord(&self, coord: Coordinate, direction: Direction) -> Option<Coordinate> {
        let (row, col) = coord;

        match direction {
            Direction::North => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Direction::South => {
                if row == self.height - 1 {
                    None
                } else {
                    Some((row + 1, col))
                }
            }
            Direction::West => {
                if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
            Direction::East => {
                if col == self.width - 1 {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
        }
    }

    /// Count how many tiles are closed inside the pipemap loop
    ///
    /// We do this by assuming the upper right corner is outisde the loop, then go through each row
    /// by row and keep track of when we transition in to and out of the loop, counting each tile
    /// we come across while inside the loop.
    ///
    /// NOTE: This assumes that all tiles that are not a part of the loop have been replaced with
    /// just a Pipe::None
    fn count_internal_tiles(&self) -> Result<usize, AdventError> {
        let mut in_loop = false;
        let mut count = 0;
        let mut prev_corner: Option<&Pipe> = None;

        for x in 0..self.height {
            for y in 0..self.width {
                let node = self.get_node((x, y))?;
                match node {
                    // Note: I know that the start in my input is a vertical pipe
                    Pipe::Vertical | Pipe::Start => in_loop = !in_loop,
                    Pipe::Horizontal => {}
                    Pipe::CornerNorthEast | Pipe::CornerSouthEast => {
                        prev_corner = Some(node);
                    }
                    Pipe::CornerNorthWest => {
                        if let Some(prev_corner) = prev_corner {
                            if prev_corner == &Pipe::CornerSouthEast {
                                in_loop = !in_loop;
                            }
                        }
                    }
                    Pipe::CornerSouthWest => {
                        if let Some(prev_corner) = prev_corner {
                            if prev_corner == &Pipe::CornerNorthEast {
                                in_loop = !in_loop;
                            }
                        }
                    }
                    Pipe::None => {
                        if in_loop {
                            count += 1;
                        }
                    }
                }
            }
        }

        Ok(count)
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32, AdventError> {
    let map: PipeMap = input.parse()?;
    let start_coord = map.find_start()?;
    let start_directions = map.get_start_directions()?;
    let mut steps = 1;

    let mut a_coord = map
        .shift_coord(start_coord, start_directions[0].clone())
        .ok_or(generic_error!(
            "Invalid start coordinate: {:?}",
            start_coord
        ))?;
    let mut a_from_direction = start_directions[0].opposite();

    let mut b_coord = map
        .shift_coord(start_coord, start_directions[1].clone())
        .ok_or(generic_error!(
            "Invalid start coordinate: {:?}",
            start_coord
        ))?;
    let mut b_from_direction = start_directions[1].opposite();

    // We'll continue stepping in each direction until they converge
    while a_coord != b_coord {
        (a_coord, a_from_direction) = map.get_next_node(a_coord, a_from_direction.clone())?;
        (b_coord, b_from_direction) = map.get_next_node(b_coord, b_from_direction.clone())?;

        steps += 1;
    }
    Ok(steps)
}

fn part2(input: &str) -> Result<usize, AdventError> {
    let map: PipeMap = input.parse()?;

    // Create an empty map to fill with _just_ the loop
    let mut clean_map = PipeMap {
        nodes: vec![vec![Pipe::None; map.width]; map.height],
        height: map.height,
        width: map.width,
    };

    // Thread the map until we reach the start again
    let start_coord = map.find_start()?;
    clean_map.nodes[start_coord.0][start_coord.1] = Pipe::Start;

    let start_directions = map.get_start_directions()?;
    let direction = start_directions[0].clone(); // We'll just pick one direction

    let mut current_coord =
        map.shift_coord(start_coord, direction.clone())
            .ok_or(generic_error!(
                "Invalid start coordinate: {:?}",
                start_coord
            ))?;
    let mut from_direction = direction.opposite();

    while current_coord != start_coord {
        let current_node = map.get_node(current_coord)?;
        clean_map.nodes[current_coord.0][current_coord.1] = current_node.clone();

        (current_coord, from_direction) = map.get_next_node(current_coord, from_direction)?;
    }

    clean_map.count_internal_tiles()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 8);
    }

    #[test]
    fn test_pipemap_find_start() {
        let map: PipeMap = PART_1_TEST_INPUT.parse().unwrap();

        assert_eq!(map.find_start().unwrap(), (2, 0));
    }

    #[test]
    fn test_pipemap_get_next_node() {
        let map: PipeMap = PART_1_TEST_INPUT.parse().unwrap();

        let node = (1, 1);

        let (next_node, came_from) = map.get_next_node(node, Direction::South).unwrap();
        assert_eq!(next_node, (1, 2));
        assert_eq!(came_from, Direction::West);

        let (next_node, came_from) = map.get_next_node(node, Direction::East).unwrap();
        assert_eq!(next_node, (2, 1));
        assert_eq!(came_from, Direction::North);
    }

    #[test]
    fn test_get_start_directions() {
        let map: PipeMap = PART_1_TEST_INPUT.parse().unwrap();

        let directions = map.get_start_directions().unwrap();

        assert_eq!(directions, vec![Direction::South, Direction::East]);
    }
}
