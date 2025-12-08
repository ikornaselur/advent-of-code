use advent::prelude::*;
use itertools::Itertools;
use parse::parse_input;
use std::env;
use std::fs;

mod parse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Paper,
}

/// A grid grows DOWN then RIGHT, meaning that coordinates are row,column with 0,0 in the upper
/// left
#[derive(Debug, PartialEq)]
pub struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(grid: Vec<Vec<Cell>>) -> Self {
        Self { grid }
    }

    pub fn get_cell(&self, row: i32, col: i32) -> Option<Cell> {
        if row < 0 || col < 0 || row >= self.grid.len() as i32 || col >= self.grid[0].len() as i32 {
            None
        } else {
            Some(self.grid[row as usize][col as usize])
        }
    }

    pub fn surround_paper_count(&self, row: i32, col: i32) -> usize {
        let deltas = [-1, 0, 1];

        deltas
            .iter()
            .cartesian_product(deltas.iter())
            .fold(0, |acc, (row_delta, col_delta)| {
                acc + match self.get_cell(row + row_delta, col + col_delta) {
                    Some(Cell::Paper) if (*row_delta != 0 || *col_delta != 0) => 1,
                    _ => 0,
                }
            })
    }

    pub fn remove_paper(&mut self, row: usize, col: usize) {
        // We're just going to cowboy this and do no checks, for the sake of SPEEED
        // Don't do this at home kids
        self.grid[row][col] = Cell::Empty;
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2025, 4)?
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
    let grid = parse_input(input)?;
    let count = (0..grid.grid.len())
        .cartesian_product(0..grid.grid[0].len())
        .fold(0, |acc, (row, col)| {
            acc + match grid.get_cell(row as i32, col as i32) {
                Some(Cell::Paper) if grid.surround_paper_count(row as i32, col as i32) < 4 => 1,
                _ => 0,
            }
        });

    Ok(count)
}

fn part2(input: &str) -> Result<usize> {
    let mut grid = parse_input(input)?;

    let mut removed = 0;
    // We're going naive brute force - it feels like that shouldn't work, but .. this problem
    // _seems_ trivial, ish? I'm scared to assume that but it's day 4
    loop {
        let removables = (0..grid.grid.len())
            .cartesian_product(0..grid.grid[0].len())
            .fold(Vec::new(), |mut acc, (row, col)| {
                match grid.get_cell(row as i32, col as i32) {
                    Some(Cell::Paper) if grid.surround_paper_count(row as i32, col as i32) < 4 => {
                        acc.push((row, col))
                    }
                    _ => {}
                };
                acc
            });

        // We don't need some flood-fill algo or something similar to optimise here, right? We can
        // remove paper that is in the middle, as long as it's not surrounded?
        // What if we need to make a graph and basically just remove all nodes with < 4 edges each
        // round, that feels like it might make sense.
        //
        // But guess what? We're brute forcing this today

        if removables.is_empty() {
            break;
        }

        removables
            .iter()
            .for_each(|(row, col)| grid.remove_paper(*row, *col));
        removed += removables.len();

        // There's some good news and some bad news.
        // The good news is that it worked! Naive brute force got us there no prob.
        // Bad news? It's slow. We're talking about four thousand and three hundred microseconds.
        // you read that right! 4.3ms, that's AGES.
        // We should be able to get this down to hundreds of microseconds.
        //
        // But that's for a later day..
    }

    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 43);
    }

    #[test]
    fn test_grid_get_cell() {
        let grid = parse_input(TEST_INPUT).unwrap();

        assert_eq!(grid.get_cell(-1, 0), None);
        assert_eq!(grid.get_cell(0, -1), None);
        assert_eq!(grid.get_cell(0, 0), Some(Cell::Empty));
        assert_eq!(grid.get_cell(0, 1), Some(Cell::Empty));
        assert_eq!(grid.get_cell(1, 0), Some(Cell::Paper));
    }

    #[test]
    fn test_grid_surround_paper_count() {
        let grid = parse_input(TEST_INPUT).unwrap();

        assert_eq!(grid.surround_paper_count(0, 2), 3);
    }
}
