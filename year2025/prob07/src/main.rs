use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fs;

mod parse;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Node {
    Start,
    Beam,
    Empty,
    Splitter,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Start => write!(f, "S"),
            Node::Beam => write!(f, "|"),
            Node::Empty => write!(f, "."),
            Node::Splitter => write!(f, "^"),
        }
    }
}

type Nodes = Vec<Node>;
type Col = usize;
type Row = usize;
type Coord = (Row, Col);

#[derive(PartialEq)]
struct Quantum(bool);

#[allow(dead_code)]
struct Grid {
    inner: Vec<Nodes>,
    start: Coord,
    width: usize,
    height: usize,
    memo_grid: Vec<Option<usize>>,
}

impl Grid {
    #[allow(unused)]
    fn height(&self) -> usize {
        self.inner.len()
    }

    fn width(&self) -> usize {
        self.inner[0].len()
    }

    #[inline]
    #[allow(dead_code)]
    fn contains_key(&self, row: usize, col: usize) -> bool {
        self.memo_grid[row + col * self.height].is_some()
    }

    #[inline]
    fn set(&mut self, row: usize, col: usize, value: usize) {
        self.memo_grid[row + col * self.height] = Some(value);
    }

    #[inline]
    fn get(&self, row: usize, col: usize) -> Option<usize> {
        self.memo_grid[row + col * self.height]
    }

    fn get_beam_splits(&mut self, (row, col): Coord, quantum: &Quantum) -> Result<usize> {
        match (quantum, self.get(row, col)) {
            (Quantum(false), Some(_)) => {
                // We return no more splits, we've been here already
                return Ok(0);
            }
            (Quantum(true), Some(splits)) => {
                // We return the splits that have been observed from here, since we're in a
                // different universe but just reached it from a different path
                // TODO: Do I need to .. store the paths?
                return Ok(splits);
            }
            (_, None) => {} // We continue
        }

        let coord_below = (row + 1, col);

        let new_splits = match self.get_node_at_coord(&coord_below) {
            Some(Node::Empty) => {
                if *quantum == Quantum(false) {
                    self.set_node_at_coord(&coord_below, Node::Beam)?;
                }
                self.get_beam_splits(coord_below, quantum)?
            }
            Some(Node::Splitter) => {
                // We split to the sides
                // NOTE: beam_col - 1 if beam_col is 0 is bad due to usize
                let left_splits = if col > 0 {
                    let coord_left = (row + 1, col - 1);
                    if *quantum == Quantum(false) {
                        self.set_node_at_coord(&coord_left, Node::Beam)?;
                    }
                    self.get_beam_splits(coord_left, quantum)?
                } else {
                    0
                };
                let right_splits = if col < self.width() - 1 {
                    let coord_right = (row + 1, col + 1);
                    if *quantum == Quantum(false) {
                        self.set_node_at_coord(&coord_right, Node::Beam)?;
                    }
                    self.get_beam_splits(coord_right, quantum)?
                } else {
                    0
                };
                match quantum {
                    Quantum(false) => left_splits + right_splits + 1, // Count number of splits
                    // plus the current one
                    Quantum(true) => left_splits + right_splits, // We're counting worlds, so we
                                                                 // don't count the split that
                                                                 // happened here
                }
            }
            Some(Node::Beam) => {
                match quantum {
                    Quantum(false) => 0, // Ignore beam already there
                    Quantum(true) => unimplemented!("Quantum"),
                }
            }
            None => match quantum {
                Quantum(false) => 0, // No more splits
                Quantum(true) => 1,  // When we go out of bounds in the quantum world, we count 1
                                      // for the number of world at this point
            },
            _ => unimplemented!(),
        };

        self.set(row, col, new_splits);

        Ok(new_splits)
    }

    fn get_node_at_coord(&self, (row, col): &Coord) -> Option<&Node> {
        self.inner.get(*row).and_then(|row| row.get(*col))
    }

    fn set_node_at_coord(&mut self, (row, col): &Coord, node: Node) -> Result<()> {
        let inner_node = self
            .inner
            .get_mut(*row)
            .and_then(|row| row.get_mut(*col))
            .ok_or_else(|| error!("Coords out of bounds"))?;

        *inner_node = node;

        Ok(())
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            for node in row {
                write!(f, "{}", node)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<Vec<Nodes>> for Grid {
    type Error = AdventError;

    fn try_from(value: Vec<Nodes>) -> Result<Self> {
        // Find the start and set it as the active beam
        // We know that it's going to be on the first row.. so we just look at the first row
        let height = value.len();
        let width = value[0].len();

        let start_row = 0;
        let start_col = value[0]
            .iter()
            .position(|node| *node == Node::Start)
            .ok_or(error!("Unable to find Start in first row"))?;

        Ok(Self {
            inner: value,
            start: (start_row, start_col),
            height,
            width,
            memo_grid: vec![None; height * width],
        })
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2025, 7)?
    };

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

fn part1(input: &str) -> Result<usize> {
    let node_rows = parse_input(input)?;
    let mut grid = Grid::try_from(node_rows)?;

    let beam_splits = grid.get_beam_splits(grid.start, &Quantum(false))?;

    Ok(beam_splits)
}

fn part2(input: &str) -> Result<usize> {
    let node_rows = parse_input(input)?;
    let mut grid = Grid::try_from(node_rows)?;

    let beam_splits = grid.get_beam_splits(grid.start, &Quantum(true))?;

    // Since each split creates a new world, and we start in one, then we +1?
    Ok(beam_splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 40);
    }

    const ONE_SPLIT_INPUT: &str = "...S...\n.......\n...^...\n.......";

    #[test]
    fn test_part1_one_split() {
        assert_eq!(part1(ONE_SPLIT_INPUT).unwrap(), 1);
    }

    #[test]
    fn test_part2_one_split() {
        // One split splits the timeline into *two* timelines
        assert_eq!(part2(ONE_SPLIT_INPUT).unwrap(), 2);
    }

    const THREE_SPLIT_INPUT: &str = "..S..\n.....\n..^..\n.....\n.^.^.\n.....";

    #[test]
    fn test_part1_three_split() {
        assert_eq!(part1(THREE_SPLIT_INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part2_three_split() {
        assert_eq!(part2(THREE_SPLIT_INPUT).unwrap(), 4);
    }

    const SIX_SPLIT_INPUT: &str =
        "...S...\n.......\n...^...\n.......\n..^.^..\n.......\n.^.^.^.\n.......\n";

    #[test]
    fn test_part1_six_split() {
        assert_eq!(part1(SIX_SPLIT_INPUT).unwrap(), 6);
    }

    #[test]
    fn test_part2_six_split() {
        assert_eq!(part2(SIX_SPLIT_INPUT).unwrap(), 8);
    }
}
