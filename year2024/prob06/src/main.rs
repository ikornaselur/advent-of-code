use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Open,
    Obsticle,
    Guard,
}

struct Grid {
    nodes: Vec<Vec<Node>>,
    width: i32,
    height: i32,
    guard: Coordinate<i32>,
    guard_direction: GridDirection,
}

fn rotate(direction: GridDirection) -> GridDirection {
    match direction {
        GridDirection::Up => GridDirection::Right,
        GridDirection::Right => GridDirection::Down,
        GridDirection::Down => GridDirection::Left,
        GridDirection::Left => GridDirection::Up,
        _ => panic!("Invalid direction"),
    }
}

impl Grid {
    fn walk_step(&mut self) -> Option<Coordinate<i32>> {
        for _ in 0..4 {
            // Move the guard in the direction it's facing
            let new_pos = match self.guard_direction {
                GridDirection::Up => (self.guard.0 - 1, self.guard.1),
                GridDirection::Down => (self.guard.0 + 1, self.guard.1),
                GridDirection::Left => (self.guard.0, self.guard.1 - 1),
                GridDirection::Right => (self.guard.0, self.guard.1 + 1),
                _ => panic!("Invalid direction"),
            };

            // Check if we are out of bounds
            if new_pos.0 < 0 || new_pos.0 >= self.height || new_pos.1 < 0 || new_pos.1 >= self.width
            {
                return None;
            }

            // Check if there's an obstacle
            if self.nodes[new_pos.0 as usize][new_pos.1 as usize] == Node::Obsticle {
                // Rotate and continue
                self.guard_direction = rotate(self.guard_direction);
            } else {
                self.guard = new_pos;
                return Some(new_pos);
            }
        }

        panic!("Guard is stuck");
    }

    #[allow(dead_code)]
    fn print_grid(&self, visited_nodes: &HashSet<Coordinate<i32>>) {
        let mut y = 0;
        for row in &self.nodes {
            let mut x = 0;
            for node in row {
                let guard_dir = match self.guard_direction {
                    GridDirection::Up => '^',
                    GridDirection::Down => 'v',
                    GridDirection::Left => '<',
                    GridDirection::Right => '>',
                    _ => panic!("Invalid direction"),
                };
                if self.guard == (y, x) {
                    print!("{}", guard_dir);
                    x += 1;
                    continue;
                }
                if visited_nodes.contains(&(y, x)) {
                    print!("X");
                    x += 1;
                    continue;
                }
                match node {
                    Node::Open => print!("."),
                    Node::Obsticle => print!("#"),
                    Node::Guard => print!("{}", guard_dir),
                }
                x += 1;
            }
            println!();
            y += 1;
        }
        println!();
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut grid = parse_input(input)?;
    // Let's just start with the naive way of traversing one by one.. that should be fast enough
    // this early on, though we can absolutely optimise long direct paths
    // Some ideas:
    // * Store obsticles in a x/y and y/x map so that we can quickly see obstacles in rows and
    // columns
    // * ... hmm, that's about it for now
    let mut visited_nodes: HashSet<Coordinate<i32>> = HashSet::new();
    visited_nodes.insert(grid.guard);

    while let Some(next_pos) = grid.walk_step() {
        visited_nodes.insert(next_pos);
    }

    Ok(visited_nodes.len())
}

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
