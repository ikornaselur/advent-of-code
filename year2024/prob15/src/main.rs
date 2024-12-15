use advent::coordinates::GridCoordinate;
use advent::prelude::*;
use parse::parse_input;
use std::fmt;

mod parse;

const INPUT: &str = include_str!("../input.txt");

type Instructions = Vec<GridDirection>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Wall,
    Box,
    Floor,
    Robot,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Node::Wall => '#',
            Node::Box => 'O',
            Node::Floor => '.',
            Node::Robot => '@',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    instructions: Vec<GridDirection>,
    idx: usize,
    robot: GridCoordinate<i32>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.nodes.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if self.robot == GridCoordinate::new(y as i32, x as i32) {
                    write!(f, "{}", Node::Robot)?;
                } else {
                    write!(f, "{}", node)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(nodes: Vec<Vec<Node>>, instructions: Vec<GridDirection>) -> Self {
        let robot = nodes
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, &node)| {
                    if node == Node::Robot {
                        Some(GridCoordinate::new(y as i32, x as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap(); // TODO: Handle this better

        // We'll just replace the robot with a 'Floor' in the actual nodes, to skip having to move
        // the robot node itself
        // TODO: Combine with previous into one pass? Probably an overkill for AoC.. but future
        // improvements
        let nodes = nodes
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&node| {
                        if node == Node::Robot {
                            Node::Floor
                        } else {
                            node
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            nodes,
            instructions,
            idx: 0,
            robot,
        }
    }

    fn tick(&mut self) -> Result<()> {
        if self.idx >= self.instructions.len() {
            return Err(error!("End of instructions"));
        }

        let vector: (i32, i32) = match self.instructions[self.idx] {
            GridDirection::Up => (-1, 0),
            GridDirection::Down => (1, 0),
            GridDirection::Left => (0, -1),
            GridDirection::Right => (0, 1),
            _ => unreachable!(),
        };

        // We'll look in this direction until we find a wall or floor, keeping track of how many
        // boxes are along the way
        let mut box_count = 0;
        let mut next_pos = self.robot;

        loop {
            next_pos += vector;

            match next_pos.get(&self.nodes) {
                Some(Node::Wall) => {
                    // We've hit a wall, we can't move, we just abort
                    break;
                }
                Some(Node::Floor) => {
                    // We've found a floor, we can move
                    if box_count == 0 {
                        // We just move the robot, don't have to worry about boxes
                        self.robot += vector;
                    } else {
                        // We basically just have to swap the first box with the floor we found
                        self.nodes[next_pos.row as usize][next_pos.column as usize] = Node::Box;
                        self.robot += vector;
                        self.nodes[self.robot.row as usize][self.robot.column as usize] =
                            Node::Floor;
                    }
                    break;
                }
                Some(Node::Box) => {
                    // We've hit a box, we'll continue moving
                    box_count += 1;
                }
                Some(Node::Robot) | None => {
                    // We know the robot isn't on the grid (as we remove it in 'new') and the
                    // problem grid has a wall surrounding the edge, so we can't get here!
                    unreachable!();
                }
            }
        }

        self.idx += 1;

        Ok(())
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, INPUT)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, INPUT)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        INPUT,
    );

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut grid = parse_input(input)?;

    while grid.tick().is_ok() {}

    Ok(grid
        .nodes
        .iter()
        .enumerate()
        .fold(0, |acc, (row_num, row)| {
            acc + row.iter().enumerate().fold(0, |acc, (column, &node)| {
                if node == Node::Box {
                    acc + column + (row_num * 100)
                } else {
                    acc
                }
            })
        }))
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const BIG_TEST_INPUT: &str = include_str!("../big_test.txt");

    #[test]
    fn test_part1_small() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 2028);
    }

    #[test]
    fn test_part1_big() {
        assert_eq!(part1(BIG_TEST_INPUT).unwrap(), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
