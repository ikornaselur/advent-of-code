use advent_core::error::AdventError;
use advent_core::generic_error;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

struct Platform {
    nodes: Vec<Vec<Node>>,
}

impl Platform {
    /// Tilt the platform in the given direction.
    ///
    /// Tilting a platform will cause all RoundRock to slide until they reach the edge or hit
    /// another rock
    fn tilt_platform(&mut self, direction: Direction) -> Result<(), AdventError> {
        // TODO: Support other direction than North - maybe by rotating the platform?
        if direction != Direction::North {
            return Err(generic_error!("Only North direction is supported"));
        }

        let row_count = self.nodes.len();
        let col_count = self.nodes[0].len();

        for row_idx in 0..row_count {
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

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq)]
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

    platform.tilt_platform(Direction::North)?;

    Ok(platform.get_load())
}

fn part2(input: &str) -> Result<u32, AdventError> {
    Ok(0)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
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
    fn test_platform_tilt() {
        let mut platform: Platform = "O..#\n....\n#OO#".parse().unwrap();
        platform.tilt_platform(Direction::North).unwrap();

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
    fn test_platform_load() {
        let mut platform: Platform = "O..#\n....\n#OO#".parse().unwrap();

        assert_eq!(platform.get_load(), 5);

        platform.tilt_platform(Direction::North).unwrap();

        assert_eq!(platform.get_load(), 9);
    }
}
