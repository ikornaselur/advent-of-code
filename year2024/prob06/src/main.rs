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

#[derive(Debug, Clone, PartialEq, Eq)]
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

fn apply_movement(position: Coordinate<i32>, direction: GridDirection) -> Coordinate<i32> {
    match direction {
        GridDirection::Up => (position.0 - 1, position.1),
        GridDirection::Down => (position.0 + 1, position.1),
        GridDirection::Left => (position.0, position.1 - 1),
        GridDirection::Right => (position.0, position.1 + 1),
        _ => panic!("Invalid direction"),
    }
}

impl Grid {
    fn walk_step(&mut self) -> bool {
        // Move the guard in the direction it's facing
        let new_pos = apply_movement(self.guard, self.guard_direction);

        // Check if we are out of bounds
        if !self.within_bounds(new_pos) {
            return false;
        }

        // Check if there's an obstacle
        if self.nodes[new_pos.0 as usize][new_pos.1 as usize] == Node::Obsticle {
            // Rotate and continue
            self.guard_direction = rotate(self.guard_direction);
        } else {
            self.guard = new_pos;
        }
        true
    }

    fn within_bounds(&self, position: Coordinate<i32>) -> bool {
        position.0 >= 0 && position.0 < self.height && position.1 >= 0 && position.1 < self.width
    }

    fn get_node(&self, position: Coordinate<i32>) -> Option<Node> {
        if !self.within_bounds(position) {
            return None;
        }
        Some(self.nodes[position.0 as usize][position.1 as usize])
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

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
    // Let's just start with the naive way of traversing one by one.. that should be fast enough
    // this early on, though we can absolutely optimise long direct paths
    // Some ideas:
    // * Store obsticles in a x/y and y/x map so that we can quickly see obstacles in rows and
    // columns
    // * ... hmm, that's about it for now
    let mut visited_nodes: HashSet<Coordinate<i32>> = HashSet::from_iter(vec![grid.guard]);

    while grid.walk_step() {
        visited_nodes.insert(grid.guard);
    }

    Ok(visited_nodes.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut grid = parse_input(input)?;

    let mut visited_nodes = HashSet::new();
    let mut count = 0;

    while grid.walk_step() {
        visited_nodes.insert(grid.guard);

        // Let's check if there's an open space in front, if so, fill it with an obsticle and
        // simulate to look for a loop
        let forward_pos = apply_movement(grid.guard, grid.guard_direction);
        if !visited_nodes.contains(&forward_pos) {
            if let Some(Node::Open) = grid.get_node(forward_pos) {
                grid.nodes[forward_pos.0 as usize][forward_pos.1 as usize] = Node::Obsticle;
                let current_pos = grid.guard;
                let current_dir = grid.guard_direction;

                let mut visited_nodes: HashSet<(Coordinate<i32>, GridDirection)> =
                    HashSet::from_iter(vec![(grid.guard, grid.guard_direction)]);
                while grid.walk_step() {
                    if visited_nodes.contains(&(grid.guard, grid.guard_direction)) {
                        count += 1;
                        break;
                    }
                    visited_nodes.insert((grid.guard, grid.guard_direction));
                }

                grid.guard = current_pos;
                grid.guard_direction = current_dir;
                grid.nodes[forward_pos.0 as usize][forward_pos.1 as usize] = Node::Open;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    fn get_grid() -> Grid {
        Grid {
            nodes: vec![
                vec![
                    Node::Open,
                    Node::Obsticle,
                    Node::Open,
                    Node::Open,
                    Node::Open,
                ],
                vec![
                    Node::Open,
                    Node::Open,
                    Node::Open,
                    Node::Open,
                    Node::Obsticle,
                ],
                vec![Node::Open, Node::Guard, Node::Open, Node::Open, Node::Open],
                vec![Node::Open, Node::Open, Node::Open, Node::Open, Node::Open],
                vec![Node::Open, Node::Open, Node::Open, Node::Open, Node::Open],
            ],
            width: 5,
            height: 5,
            guard: (2, 1),
            guard_direction: GridDirection::Up,
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 6);
    }

    #[test]
    fn test_grid_get_node() {
        // This should be the obsticle in the second row, as the coordinates are (y, x), or, (row, col)
        let grid = get_grid();
        let pos = (1, 4);
        assert_eq!(grid.get_node(pos), Some(Node::Obsticle));
    }
}
