use advent::prelude::*;

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

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Horizontal),
            "|" => Ok(Self::Vertical),
            "L" => Ok(Self::CornerNorthEast),
            "J" => Ok(Self::CornerNorthWest),
            "F" => Ok(Self::CornerSouthEast),
            "7" => Ok(Self::CornerSouthWest),
            "S" => Ok(Self::Start),
            "." => Ok(Self::None),
            _ => Err(error!("Invalid pipe: {}", s)),
        }
    }
}

impl Pipe {
    fn connects_to(&self) -> HashSet<CompassDirection> {
        match self {
            Pipe::Horizontal => vec![CompassDirection::East, CompassDirection::West],
            Pipe::Vertical => vec![CompassDirection::North, CompassDirection::South],
            Pipe::CornerNorthEast => vec![CompassDirection::North, CompassDirection::East],
            Pipe::CornerNorthWest => vec![CompassDirection::North, CompassDirection::West],
            Pipe::CornerSouthEast => vec![CompassDirection::South, CompassDirection::East],
            Pipe::CornerSouthWest => vec![CompassDirection::South, CompassDirection::West],
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

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<Pipe>())
                    .collect::<std::result::Result<Vec<Pipe>, Self::Err>>()
            })
            .collect::<std::result::Result<Vec<Vec<Pipe>>, Self::Err>>()?;

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
    fn find_start(&self) -> Result<GridCoordinate<usize>> {
        for (x, row) in self.nodes.iter().enumerate() {
            for (y, node) in row.iter().enumerate() {
                if *node == Pipe::Start {
                    return Ok(GridCoordinate { row: x, column: y });
                }
            }
        }
        Err(error!("Start node not found"))
    }

    /// Look at the nodes around to see which node is the next one
    fn get_next_node(
        &self,
        current_node_coord: GridCoordinate<usize>,
        came_from_direction: CompassDirection,
    ) -> Result<(GridCoordinate<usize>, CompassDirection)> {
        let current_node = current_node_coord
            .get(&self.nodes)
            .ok_or(error!("Out of bounds"))?;

        if *current_node == Pipe::None || *current_node == Pipe::Start {
            return Err(error!("Invalid node: {:?}", current_node));
        }

        // Get the next direction for this pipe
        let binding = &vec![came_from_direction].into_iter().collect();
        let connects_to = current_node.connects_to();
        let next_direction = connects_to
            .difference(binding)
            .next()
            .ok_or_else(|| error!("No next direction found for {:?}", current_node))?;

        // Check if that next direction is valid
        let next_node_coord = match next_direction {
            CompassDirection::North => GridCoordinate {
                row: current_node_coord.row - 1,
                column: current_node_coord.column,
            },
            CompassDirection::South => GridCoordinate {
                row: current_node_coord.row + 1,
                column: current_node_coord.column,
            },
            CompassDirection::West => GridCoordinate {
                row: current_node_coord.row,
                column: current_node_coord.column - 1,
            },
            CompassDirection::East => GridCoordinate {
                row: current_node_coord.row,
                column: current_node_coord.column + 1,
            },
            _ => panic!("Bad direction"),
        };

        if next_node_coord.row >= self.height || next_node_coord.column >= self.width {
            return Err(error!(
                "Invalid next node coordinate: {:?}",
                next_node_coord
            ));
        }

        let next_node = next_node_coord
            .get(&self.nodes)
            .ok_or(error!("Out of bounds"))?;
        if *next_node == Pipe::None {
            return Err(error!("Invalid next node: {:?}", next_node));
        }

        Ok((next_node_coord, next_direction.clone().opposite()))
    }

    // Get the two directions that connect to the start
    fn get_start_directions(&self) -> Result<Vec<CompassDirection>> {
        let start_coord = self.find_start()?;
        let mut directions = vec![];

        // Check the four nodes around to see if any of them connect back

        // North node
        if let Some(north_coord) = self.shift_coord(start_coord, CompassDirection::North) {
            let north_node = north_coord.get(&self.nodes).unwrap();
            if north_node.connects_to().contains(&CompassDirection::South) {
                directions.push(CompassDirection::North);
            }
        }

        // South node
        if let Some(south_coord) = self.shift_coord(start_coord, CompassDirection::South) {
            let south_node = south_coord.get(&self.nodes).unwrap();
            if south_node.connects_to().contains(&CompassDirection::North) {
                directions.push(CompassDirection::South);
            }
        }

        // West node
        if let Some(west_coord) = self.shift_coord(start_coord, CompassDirection::West) {
            let west_node = west_coord.get(&self.nodes).unwrap();
            if west_node.connects_to().contains(&CompassDirection::East) {
                directions.push(CompassDirection::West);
            }
        }

        // East node
        if let Some(east_coord) = self.shift_coord(start_coord, CompassDirection::East) {
            let east_node = east_coord.get(&self.nodes).unwrap();
            if east_node.connects_to().contains(&CompassDirection::West) {
                directions.push(CompassDirection::East);
            }
        }

        if directions.len() != 2 {
            return Err(error!(
                "Invalid number of directions found: {:?} - expected 2!",
                directions
            ));
        }

        Ok(directions)
    }

    fn shift_coord(
        &self,
        coord: GridCoordinate<usize>,
        direction: CompassDirection,
    ) -> Option<GridCoordinate<usize>> {
        match direction {
            CompassDirection::North => {
                if coord.row == 0 {
                    None
                } else {
                    Some(GridCoordinate {
                        row: coord.row - 1,
                        column: coord.column,
                    })
                }
            }
            CompassDirection::South => {
                if coord.row == self.height - 1 {
                    None
                } else {
                    Some(GridCoordinate {
                        row: coord.row + 1,
                        column: coord.column,
                    })
                }
            }
            CompassDirection::West => {
                if coord.column == 0 {
                    None
                } else {
                    Some(GridCoordinate {
                        row: coord.row,
                        column: coord.column - 1,
                    })
                }
            }
            CompassDirection::East => {
                if coord.column == self.width - 1 {
                    None
                } else {
                    Some(GridCoordinate {
                        row: coord.row,
                        column: coord.column + 1,
                    })
                }
            }
            _ => panic!("Bad direction"),
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
    fn count_internal_tiles(&self) -> Result<usize> {
        let mut in_loop = false;
        let mut count = 0;
        let mut prev_corner: Option<&Pipe> = None;

        for x in 0..self.height {
            for y in 0..self.width {
                let node = GridCoordinate { row: x, column: y }
                    .get(&self.nodes)
                    .ok_or(error!("Out of bounds"))?;

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

fn main() -> Result<()> {
    let input = get_input(2023, 10)?;

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

fn part1(input: &str) -> Result<u32> {
    let map: PipeMap = input.parse()?;
    let start_coord = map.find_start()?;
    let start_directions = map.get_start_directions()?;
    let mut steps = 1;

    let mut a_coord = map
        .shift_coord(start_coord, start_directions[0])
        .ok_or(error!("Invalid start coordinate: {:?}", start_coord))?;
    let mut a_from_direction = start_directions[0].opposite();

    let mut b_coord = map
        .shift_coord(start_coord, start_directions[1])
        .ok_or(error!("Invalid start coordinate: {:?}", start_coord))?;
    let mut b_from_direction = start_directions[1].opposite();

    // We'll continue stepping in each direction until they converge
    while a_coord != b_coord {
        (a_coord, a_from_direction) = map.get_next_node(a_coord, a_from_direction)?;
        (b_coord, b_from_direction) = map.get_next_node(b_coord, b_from_direction)?;

        steps += 1;
    }
    Ok(steps)
}

fn part2(input: &str) -> Result<usize> {
    let map: PipeMap = input.parse()?;

    // Create an empty map to fill with _just_ the loop
    let mut clean_map = PipeMap {
        nodes: vec![vec![Pipe::None; map.width]; map.height],
        height: map.height,
        width: map.width,
    };

    // Thread the map until we reach the start again
    let start_coord = map.find_start()?;
    start_coord.set(&mut clean_map.nodes, Pipe::Start)?;

    let start_directions = map.get_start_directions()?;
    let direction = start_directions[0]; // We'll just pick one direction

    let mut current_coord = map
        .shift_coord(start_coord, direction)
        .ok_or(error!("Invalid start coordinate: {:?}", start_coord))?;
    let mut from_direction = direction.opposite();

    while current_coord != start_coord {
        let current_node = current_coord.get(&map.nodes).unwrap();
        current_coord.set(&mut clean_map.nodes, current_node.clone())?;

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

        assert_eq!(
            map.find_start().unwrap(),
            GridCoordinate { row: 2, column: 0 }
        );
    }

    #[test]
    fn test_pipemap_get_next_node() {
        let map: PipeMap = PART_1_TEST_INPUT.parse().unwrap();

        let node = GridCoordinate { row: 1, column: 1 };

        let (next_node, came_from) = map.get_next_node(node, CompassDirection::South).unwrap();
        assert_eq!(next_node, GridCoordinate { row: 1, column: 2 });
        assert_eq!(came_from, CompassDirection::West);

        let (next_node, came_from) = map.get_next_node(node, CompassDirection::East).unwrap();
        assert_eq!(next_node, GridCoordinate { row: 2, column: 1 });
        assert_eq!(came_from, CompassDirection::North);
    }

    #[test]
    fn test_get_start_directions() {
        let map: PipeMap = PART_1_TEST_INPUT.parse().unwrap();

        let directions = map.get_start_directions().unwrap();

        assert_eq!(
            directions,
            vec![CompassDirection::South, CompassDirection::East]
        );
    }
}
