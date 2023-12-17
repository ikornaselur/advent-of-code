use advent::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Platform {
    nodes: Vec<Vec<Node>>,
}

impl Platform {
    /// Tilt the platform in the given direction.
    ///
    /// Tilting a platform will cause all RoundRock to slide until they reach the edge or hit
    /// another rock
    fn tilt_platform(&mut self, direction: &Direction) -> Result<(), AdventError> {
        let row_count = self.nodes.len();
        let col_count = self.nodes[0].len();

        match direction {
            Direction::North => {
                for row_idx in 1..row_count {
                    for col_idx in 0..col_count {
                        let node = &self.nodes[row_idx][col_idx];
                        if let Node::RoundRock = node {
                            // Let's see how far up we can get
                            for i in (1..=row_idx).rev() {
                                let next_node = &self.nodes[i - 1][col_idx];
                                if let Node::Space = next_node {
                                    // Swap the nodes
                                    // TODO: No need to swap at every step
                                    self.nodes[i - 1][col_idx] = Node::RoundRock;
                                    self.nodes[i][col_idx] = Node::Space;
                                } else {
                                    // We can't go further up
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Direction::South => {
                for row_idx in (0..row_count - 1).rev() {
                    for col_idx in 0..col_count {
                        let node = &self.nodes[row_idx][col_idx];
                        if let Node::RoundRock = node {
                            for i in row_idx + 1..row_count {
                                let next_node = &self.nodes[i][col_idx];
                                if let Node::Space = next_node {
                                    // Swap the nodes
                                    self.nodes[i][col_idx] = Node::RoundRock;
                                    self.nodes[i - 1][col_idx] = Node::Space;
                                } else {
                                    // We can't go further down
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Direction::West => {
                for row_idx in 0..row_count {
                    for col_idx in 1..col_count {
                        let node = &self.nodes[row_idx][col_idx];
                        if let Node::RoundRock = node {
                            for i in (1..=col_idx).rev() {
                                let next_node = &self.nodes[row_idx][i - 1];
                                if let Node::Space = next_node {
                                    // Swap the nodes
                                    self.nodes[row_idx][i - 1] = Node::RoundRock;
                                    self.nodes[row_idx][i] = Node::Space;
                                } else {
                                    // We can't go further left
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Direction::East => {
                for row_idx in 0..row_count {
                    for col_idx in (0..col_count - 1).rev() {
                        let node = &self.nodes[row_idx][col_idx];
                        if let Node::RoundRock = node {
                            for i in col_idx + 1..col_count {
                                let next_node = &self.nodes[row_idx][i];
                                if let Node::Space = next_node {
                                    // Swap the nodes
                                    self.nodes[row_idx][i] = Node::RoundRock;
                                    self.nodes[row_idx][i - 1] = Node::Space;
                                } else {
                                    // We can't go further right
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Calculate platform load
    ///
    /// The load caused by a single RoundRock is equal to the number of rows from the south edge
    fn get_load(&self) -> usize {
        let row_count = self.nodes.len();

        let mut load = 0;

        for (idx, row) in self.nodes.iter().enumerate() {
            for node in row {
                if let Node::RoundRock = node {
                    load += row_count - idx;
                }
            }
        }

        load
    }
}

impl FromStr for Platform {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Node::from(c));
            }
            nodes.push(row);
        }

        Ok(Platform { nodes })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Node {
    RoundRock, // O
    CubeRock,  // #
    Space,     // .
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            'O' => Node::RoundRock,
            '#' => Node::CubeRock,
            '.' => Node::Space,
            _ => panic!("Invalid node type"),
        }
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
    let mut platform: Platform = input.parse()?;

    platform.tilt_platform(&Direction::North)?;

    Ok(platform.get_load())
}

fn part2(input: &str) -> Result<usize, AdventError> {
    let mut platform: Platform = input.parse()?;

    // Keep track of how many times we've seen a state, and which direction we tilted to get here
    let mut iterations = 0;
    let goal = 1_000_000_000;
    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    let mut seen: HashMap<Platform, usize> = HashMap::new();

    // let mut seen: HashSet<Platform> = HashSet::new();
    let mut loop_found = false;

    // Start rotating and store states along with the previous direction
    while iterations < goal {
        for direction in directions {
            platform.tilt_platform(&direction)?;
        }

        if !loop_found {
            if let Some(loop_start) = seen.get(&platform) {
                let loop_len = iterations - loop_start;
                let loops = (goal - iterations) / loop_len;
                iterations += (loops - 2) * loop_len;
                loop_found = true;
            }
            seen.insert(platform.clone(), iterations);
        }

        iterations += 1;
    }

    Ok(platform.get_load())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 64);
    }

    #[test]
    fn test_platform_from_str() {
        let platform: Platform = "O..#\n....\n#OO#".parse().unwrap();

        assert_eq!(platform.nodes.len(), 3);
        assert_eq!(
            platform.nodes[0],
            vec![Node::RoundRock, Node::Space, Node::Space, Node::CubeRock]
        );
        assert_eq!(
            platform.nodes[1],
            vec![Node::Space, Node::Space, Node::Space, Node::Space]
        );
        assert_eq!(
            platform.nodes[2],
            vec![
                Node::CubeRock,
                Node::RoundRock,
                Node::RoundRock,
                Node::CubeRock
            ]
        );
    }

    #[test]
    fn test_platform_tilt_north() {
        let mut platform: Platform = "O..#\n....\n#OO#".parse().unwrap();
        platform.tilt_platform(&Direction::North).unwrap();

        assert_eq!(platform.nodes.len(), 3);
        assert_eq!(
            platform.nodes[0],
            vec![
                Node::RoundRock,
                Node::RoundRock,
                Node::RoundRock,
                Node::CubeRock
            ]
        );
        assert_eq!(
            platform.nodes[1],
            vec![Node::Space, Node::Space, Node::Space, Node::Space]
        );
        assert_eq!(
            platform.nodes[2],
            vec![Node::CubeRock, Node::Space, Node::Space, Node::CubeRock]
        );
    }

    #[test]
    fn test_platform_tilt_south() {
        let mut platform: Platform = "O..#\n....\n#OO#".parse().unwrap();
        platform.tilt_platform(&Direction::South).unwrap();

        assert_eq!(platform.nodes.len(), 3);
        assert_eq!(
            platform.nodes[0],
            vec![Node::Space, Node::Space, Node::Space, Node::CubeRock]
        );
        assert_eq!(
            platform.nodes[1],
            vec![Node::RoundRock, Node::Space, Node::Space, Node::Space]
        );
        assert_eq!(
            platform.nodes[2],
            vec![
                Node::CubeRock,
                Node::RoundRock,
                Node::RoundRock,
                Node::CubeRock
            ]
        );
    }

    #[test]
    fn test_platform_tilt_west() {
        let mut platform: Platform = ".O.#\n....\n#.O#".parse().unwrap();
        platform.tilt_platform(&Direction::West).unwrap();

        assert_eq!(platform.nodes.len(), 3);
        assert_eq!(
            platform.nodes[0],
            vec![Node::RoundRock, Node::Space, Node::Space, Node::CubeRock]
        );
        assert_eq!(
            platform.nodes[1],
            vec![Node::Space, Node::Space, Node::Space, Node::Space]
        );
        assert_eq!(
            platform.nodes[2],
            vec![Node::CubeRock, Node::RoundRock, Node::Space, Node::CubeRock]
        );
    }

    #[test]
    fn test_platform_tilt_east() {
        let mut platform: Platform = "O..#\n....\n#O.#".parse().unwrap();
        platform.tilt_platform(&Direction::East).unwrap();

        assert_eq!(platform.nodes.len(), 3);
        assert_eq!(
            platform.nodes[0],
            vec![Node::Space, Node::Space, Node::RoundRock, Node::CubeRock]
        );
        assert_eq!(
            platform.nodes[1],
            vec![Node::Space, Node::Space, Node::Space, Node::Space]
        );
        assert_eq!(
            platform.nodes[2],
            vec![Node::CubeRock, Node::Space, Node::RoundRock, Node::CubeRock]
        );
    }

    #[test]
    fn test_platform_load() {
        let mut platform: Platform = "O..#\n....\n#OO#".parse().unwrap();

        assert_eq!(platform.get_load(), 5);

        platform.tilt_platform(&Direction::North).unwrap();

        assert_eq!(platform.get_load(), 9);
    }
}
