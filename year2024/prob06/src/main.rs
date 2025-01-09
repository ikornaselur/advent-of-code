use advent::prelude::*;
use parse::parse_input;

mod parse;

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
    guard: GridCoordinate<i32>,
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

fn apply_movement(position: GridCoordinate<i32>, direction: GridDirection) -> GridCoordinate<i32> {
    match direction {
        GridDirection::Up => GridCoordinate {
            row: position.row - 1,
            column: position.column,
        },
        GridDirection::Down => GridCoordinate {
            row: position.row + 1,
            column: position.column,
        },
        GridDirection::Left => GridCoordinate {
            row: position.row,
            column: position.column - 1,
        },
        GridDirection::Right => GridCoordinate {
            row: position.row,
            column: position.column + 1,
        },
        _ => panic!("Invalid direction"),
    }
}

impl Grid {
    fn walk_step(&mut self) -> bool {
        // Move the guard in the direction it's facing
        let new_pos = apply_movement(self.guard, self.guard_direction);

        // Check if we are out of bounds
        if !new_pos.within_grid(&self.nodes) {
            return false;
        }

        // Check if there's an obstacle
        if let Some(Node::Obsticle) = new_pos.get(&self.nodes) {
            // Rotate and continue
            self.guard_direction = rotate(self.guard_direction);
        } else {
            self.guard = new_pos;
        }
        true
    }
}

fn main() -> Result<()> {
    let input = get_input(2024, 6)?;

    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, &input)?;
    println!(" > {result}");

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, &input)?;
    println!(" > {result}");

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
    let mut grid = parse_input(input)?;
    // Let's just start with the naive way of traversing one by one.. that should be fast enough
    // this early on, though we can absolutely optimise long direct paths
    // Some ideas:
    // * Store obsticles in a x/y and y/x map so that we can quickly see obstacles in rows and
    // columns
    // * ... hmm, that's about it for now
    let mut visited_nodes: HashSet<GridCoordinate<i32>> = HashSet::from_iter(vec![grid.guard]);

    while grid.walk_step() {
        visited_nodes.insert(grid.guard);
    }

    Ok(visited_nodes.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut grid = parse_input(input)?;

    let mut visited_nodes =
        HashSet::with_capacity(usize::try_from(grid.width)? * usize::try_from(grid.height)?);
    let mut count = 0;

    while grid.walk_step() {
        visited_nodes.insert(grid.guard);

        // Let's check if there's an open space in front, if so, fill it with an obsticle and
        // simulate to look for a loop
        let forward_pos = apply_movement(grid.guard, grid.guard_direction);
        if !visited_nodes.contains(&forward_pos)
            && forward_pos.within_grid(&grid.nodes)
            && forward_pos
                .get(&grid.nodes)
                .ok_or(error!("Out of bounds"))?
                == &Node::Open
        {
            forward_pos.set(&mut grid.nodes, Node::Obsticle)?;
            let current_pos = grid.guard;
            let current_dir = grid.guard_direction;

            let mut visited_nodes: HashSet<(GridCoordinate<i32>, GridDirection)> =
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
            forward_pos.set(&mut grid.nodes, Node::Open)?;
        }
    }

    Ok(count)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 6);
    }
}
