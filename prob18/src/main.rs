use advent::prelude::*;
use std::fmt;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Layout {
    nodes: Vec<Vec<Node>>,
    start: Coordinate<i32>, // Where the start node is, this is used for shifting the grid if needed
}

impl Layout {
    fn from_instructions(instructions: &[Instruction]) -> Result<Self> {
        let mut current: Coordinate<i32> = (0, 0);

        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut min_y = 0;

        // Go through all the instructions to see how big the grid is
        for instruction in instructions {
            let steps = instruction.steps as i32;
            current = match instruction.direction {
                OrdinalDirection::Up => (current.0 - steps, current.1),
                OrdinalDirection::Down => (current.0 + steps, current.1),
                OrdinalDirection::Left => (current.0, current.1 - steps),
                OrdinalDirection::Right => (current.0, current.1 + steps),
            };
            max_x = max_x.max(current.0);
            max_y = max_y.max(current.1);
            min_x = min_x.min(current.0);
            min_y = min_y.min(current.1);
        }

        // Initialise the nodes to the correct size
        let height = (max_x - min_x + 1) as usize;
        let width = (max_y - min_y + 1) as usize;
        let nodes = vec![vec![Node::new(false); width]; height];

        // Start node indicates where the first node is, this is based off the min x and y
        let start = (-min_x, -min_y);

        Ok(Self { nodes, start })
    }

    fn dig(&mut self, coordinate: Coordinate<i32>) -> Result<()> {
        let (x, y) = (
            (coordinate.0 + self.start.0) as usize,
            (coordinate.1 + self.start.1) as usize,
        );

        // Grow the layout if necessary
        if self.nodes.len() <= x {
            self.nodes.resize(x + 1, vec![]);
        }
        if self.nodes[x].len() <= y {
            self.nodes[x].resize(y + 1, Node::new(false));
        }

        self.nodes[x][y].dig()?;
        self.nodes[x][y].edge = true;

        Ok(())
    }

    /// Do a flood-fill digging around this coordinate, until there are no more nodes to dig
    fn fill_from(&mut self, coordinate: Coordinate<i32>) -> Result<()> {
        let mut queue: VecDeque<Coordinate<i32>> = VecDeque::new();
        queue.push_back(coordinate);

        while let Some(coord) = queue.pop_front() {
            let node = &mut self.nodes[coord.0 as usize][coord.1 as usize];
            if node.depth > 0 {
                continue;
            }

            node.dig()?;

            // Push all the nodes around that aren't dug yet
            for direction in &[
                OrdinalDirection::Up,
                OrdinalDirection::Down,
                OrdinalDirection::Left,
                OrdinalDirection::Right,
            ] {
                let new_coord = match direction {
                    OrdinalDirection::Up => (coord.0 - 1, coord.1),
                    OrdinalDirection::Down => (coord.0 + 1, coord.1),
                    OrdinalDirection::Left => (coord.0, coord.1 - 1),
                    OrdinalDirection::Right => (coord.0, coord.1 + 1),
                };
                if self.nodes[new_coord.0 as usize][new_coord.1 as usize].depth == 0 {
                    queue.push_back(new_coord);
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Layout {
    /// Print the layout as just:
    ///     1. If depth is 0, print a .
    ///     2. If depth is > 0, print #
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x, row) in self.nodes.iter().enumerate() {
            for (y, node) in row.iter().enumerate() {
                if self.start == (x as i32, y as i32) {
                    write!(f, "X")?;
                } else if node.depth == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Node {
    depth: usize,
    edge: bool,
    colour: Option<Colour>,
}

impl Node {
    fn new(edge: bool) -> Self {
        Self {
            depth: 0,
            edge,
            colour: None,
        }
    }

    fn dig(&mut self) -> Result<()> {
        self.depth += 1;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Colour {
    hex: String,
}

impl FromStr for Colour {
    type Err = AdventError;

    /// Parse a hex colour code
    ///
    /// Input is expected to be in the form of: (#rrggbb)
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let hex = s
            .trim_start_matches('(')
            .trim_start_matches('#')
            .trim_end_matches(')')
            .to_string();
        Ok(Self { hex })
    }
}

/// An instruction is in the form of:
///
/// R 6 (#70c710)
///
/// where the first character is U/D/L/R for Up/Down/Left/Right
/// second character is number of steps
/// the third part is the hex colour code
#[derive(Debug)]
struct Instruction {
    direction: OrdinalDirection,
    steps: usize,
    colour: Colour,
}

impl FromStr for Instruction {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction =
            OrdinalDirection::from_UDLR(parts.next().ok_or(error!("Unable to parse direction"))?)
                .ok_or(error!("Unable to parse direction"))?;
        let steps = parts
            .next()
            .ok_or(error!("Unable to parse steps"))?
            .parse()?;
        let colour = parts
            .next()
            .ok_or(error!("Unable to parse colour"))?
            .parse()?;

        Ok(Self {
            direction,
            steps,
            colour,
        })
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    // We could find it dynamically .. oooor just look at the photo
    // TODO: Future enhancement
    let known_inside = (110, 63);
    println!(" > {}", part1(INPUT, known_inside)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str, known_inside: Coordinate<i32>) -> Result<usize> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Instruction>>>()?;

    let mut layout = Layout::from_instructions(&instructions)?;

    // Go through the instructions, digging each node as we go through
    let mut position: Coordinate<i32> = (0, 0);
    for instruction in instructions {
        // Walk the number of steps and dig each node
        for _ in 0..instruction.steps {
            position = match instruction.direction {
                OrdinalDirection::Up => (position.0 - 1, position.1),
                OrdinalDirection::Down => (position.0 + 1, position.1),
                OrdinalDirection::Left => (position.0, position.1 - 1),
                OrdinalDirection::Right => (position.0, position.1 + 1),
            };
            layout.dig(position)?;
        }
    }

    layout.fill_from(known_inside)?;

    // Get the volume of the layout, which is the sum of the depth of each node
    let volume = layout
        .nodes
        .iter()
        .flatten()
        .map(|n| n.depth)
        .sum::<usize>();

    Ok(volume)
}

fn part2(input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, (1, 1)).unwrap(), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_parse_instruction() {
        let instruction: Instruction = "R 16 (#70c710)".parse().unwrap();

        assert_eq!(instruction.direction, OrdinalDirection::Right);
        assert_eq!(instruction.steps, 16);
        assert_eq!(instruction.colour.hex, "70c710");
    }
}
