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

struct Grid {
    inner: Vec<Nodes>,
    active_beams: Vec<Coord>,
    beam_splits: usize,
}

impl Grid {
    #[allow(unused)]
    fn height(&self) -> usize {
        self.inner.len()
    }

    fn width(&self) -> usize {
        self.inner[0].len()
    }

    fn beam_tick(&mut self) -> Result<()> {
        // TODO: Review the clone, self.set_node_at_coord could mutably borrow just self.inner
        // instead of self
        let mut new_active_beams = Vec::new();
        for (beam_row, beam_col) in self.active_beams.clone() {
            match self.get_node_at_coord(&(beam_row, beam_col)) {
                Some(node) if *node == Node::Start || *node == Node::Beam => node,
                _ => panic!("Invalid node to tick"),
            };

            let coord_below = (beam_row + 1, beam_col);

            match self.get_node_at_coord(&coord_below) {
                Some(Node::Empty) => {
                    self.set_node_at_coord(&coord_below, Node::Beam)?;
                    new_active_beams.push(coord_below);
                }
                Some(Node::Splitter) => {
                    // We split to the sides
                    // TODO: beam_col - 1 if beam_col is 0 is bad due to usize
                    if beam_col > 0 {
                        let coord_left = (beam_row + 1, beam_col - 1);
                        self.set_node_at_coord(&coord_left, Node::Beam)?;
                        new_active_beams.push(coord_left);
                    }
                    if beam_col < self.width() - 1 {
                        let coord_right = (beam_row + 1, beam_col + 1);
                        self.set_node_at_coord(&coord_right, Node::Beam)?;
                        new_active_beams.push(coord_right);
                    }
                    self.beam_splits += 1;
                }
                Some(Node::Beam) => {} // Ignore Beam already there
                None => {}             // Ignore out of bounds?
                _ => unimplemented!(),
            }
        }
        self.active_beams = new_active_beams;
        Ok(())
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
        let start_row = 0;
        let start_col = value[0]
            .iter()
            .position(|node| *node == Node::Start)
            .ok_or(error!("Unable to find Start in first row"))?;

        Ok(Self {
            inner: value,
            active_beams: vec![(start_row, start_col)],
            beam_splits: 0,
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

    while !grid.active_beams.is_empty() {
        grid.beam_tick()?;
    }

    Ok(grid.beam_splits)
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
