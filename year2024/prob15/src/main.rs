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
    BoxLeft,
    BoxRight,
    Floor,
    Robot,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Node::Wall => '#',
            Node::Box => 'O',
            Node::BoxLeft => '[',
            Node::BoxRight => ']',
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

    fn widen_grid(&mut self) {
        // We'll need to replace each row with a row that is twice as long, the number of rows stay
        // the same.
        // We replace each node by the following rules:
        //
        //   Wall -> Wall, Wall
        //   Box -> BoxLeft, BoxRight
        //   Floor -> Floor, Floor
        self.nodes = self
            .nodes
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|&node| match node {
                        Node::Wall => vec![Node::Wall, Node::Wall],
                        Node::Box => vec![Node::BoxLeft, Node::BoxRight],
                        Node::Floor => vec![Node::Floor, Node::Floor],
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        // We'll then have to keep in mind that the robot will shift to the right, which we can
        // achieve by just doubling the column coordinate
        self.robot.column *= 2;
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

        if self.move_node(self.robot, vector, true) {
            self.robot += vector;
        }

        self.idx += 1;

        Ok(())
    }

    fn move_node(
        &mut self,
        coordinate: GridCoordinate<i32>,
        vector: (i32, i32),
        apply_move: bool,
    ) -> bool {
        let current_node = match coordinate.get(&self.nodes) {
            Some(node) => *node,
            None => panic!("Coordinate out of bounds"),
        };

        let next_pos = coordinate + vector;

        match next_pos.get(&self.nodes) {
            Some(Node::Wall) => {
                // We've hit a wall, we can't move
                false
            }
            Some(Node::Floor) => {
                // We've found a floor, we can move
                if apply_move {
                    self.nodes[next_pos.row as usize][next_pos.column as usize] = current_node;
                    self.nodes[coordinate.row as usize][coordinate.column as usize] = Node::Floor;
                }

                true
            }
            Some(Node::Box) => {
                // We've hit a box, let's see if it would move
                if self.move_node(next_pos, vector, apply_move) {
                    // It moved, we can move the current node
                    if apply_move {
                        self.nodes[next_pos.row as usize][next_pos.column as usize] = current_node;
                        self.nodes[coordinate.row as usize][coordinate.column as usize] =
                            Node::Floor;
                    }

                    true
                } else {
                    // It didn't move, we can't move
                    false
                }
            }
            Some(Node::BoxLeft) | Some(Node::BoxRight) if vector.0 == 0 => {
                // We're moving a wide box horizontally, which is just like moving any other box
                // really. The first part of the box hit will move the other half of the box, which
                // will move if there's free space.. so we just continue like normal narrow boxes
                if self.move_node(next_pos, vector, apply_move) {
                    // Then we move the left side as well
                    if apply_move {
                        self.nodes[next_pos.row as usize][next_pos.column as usize] = current_node;
                        self.nodes[coordinate.row as usize][coordinate.column as usize] =
                            Node::Floor;
                    }

                    true
                } else {
                    false
                }
            }
            Some(node @ (Node::BoxLeft | Node::BoxRight)) if vector.1 == 0 => {
                // Now we're cooking.. moving vertically, that's going to require some
                // backtracking! We'll achieve that with this this 'apply_move' flag, so that we
                // can see if *all* touched boxes would move, only then will we apply the move..
                // this _should_ prevent the issue of pushing two boxes vertically where the left
                // box can't move but the right can (if we apply the move immediately we'll be in a
                // bad state)

                // We also have to keep in mind that we're covering _two_ nodes at a time here
                let box_vector = match node {
                    Node::BoxLeft => (0, 1),
                    Node::BoxRight => (0, -1),
                    _ => unreachable!(),
                };
                let next_pos_other = next_pos + box_vector;

                if self.move_node(next_pos, vector, false)
                    && self.move_node(next_pos_other, vector, false)
                {
                    if apply_move {
                        self.move_node(next_pos, vector, true);
                        self.move_node(next_pos_other, vector, true);

                        // We then have to move both the original node
                        // I also thought I had to move the 'other half', but, somehow.. this
                        // works? If I comment this out, the tests fail, and similarly if I
                        // explicitly move the other part.. it doesn't work
                        // Sooooo..
                        self.nodes[next_pos.row as usize][next_pos.column as usize] = current_node;
                        self.nodes[coordinate.row as usize][coordinate.column as usize] =
                            Node::Floor;
                    }

                    // And we know it could move, so we can just return true!
                    true
                } else {
                    false
                }
            }
            Some(Node::Robot) | None => unreachable!(),
            _ => todo!(),
        }
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

fn part2(input: &str) -> Result<usize> {
    let mut grid = parse_input(input)?;

    grid.widen_grid();

    while grid.tick().is_ok() {}

    Ok(grid
        .nodes
        .iter()
        .enumerate()
        .fold(0, |acc, (row_num, row)| {
            acc + row.iter().enumerate().fold(0, |acc, (column, &node)| {
                if node == Node::BoxLeft {
                    acc + column + (row_num * 100)
                } else {
                    acc
                }
            })
        }))
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
        assert_eq!(part2(BIG_TEST_INPUT).unwrap(), 9021);
    }
}
