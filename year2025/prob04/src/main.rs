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
        assert_eq!(part1(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
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
