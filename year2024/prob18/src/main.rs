use advent::prelude::*;
use parse::parse_input;
use std::{env, fmt, fs};

mod parse;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2024, 18)?
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

struct Grid {
    nodes: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            nodes: vec![vec![0; width]; height],
            width,
            height,
        }
    }

    fn drop_byte(&mut self, coordinate: &GridCoordinate<i32>) -> Result<()> {
        if coordinate.row >= self.height as i32
            || coordinate.column >= self.width as i32
            || coordinate.row < 0
            || coordinate.column < 0
        {
            return Err(error!("Coordinate out of bounds"));
        }

        self.nodes[coordinate.row as usize][coordinate.column as usize] += 1;

        Ok(())
    }

    fn get_node(&self, coordinate: &GridCoordinate<i32>) -> Option<i32> {
        if coordinate.row >= self.height as i32
            || coordinate.column >= self.width as i32
            || coordinate.row < 0
            || coordinate.column < 0
        {
            return None;
        }

        Some(self.nodes[coordinate.row as usize][coordinate.column as usize])
    }

    fn get_shortest_distance(
        &mut self,
        from: GridCoordinate<i32>,
        to: GridCoordinate<i32>,
    ) -> Result<u32> {
        // Time to solve a maze again!
        // The queue is a binary heap of tuples, first element is the distance so far (negative),
        // followed by the manhattan distance between the nodes, and then followed by the
        // coordinate. We'll be doing BFS, so we don't need to keep track of the path itself...
        // though we haven't seen part 2 yet.
        let mut queue: BinaryHeap<(i32, GridCoordinate<i32>)> = BinaryHeap::new();
        let mut seen_nodes: HashSet<GridCoordinate<i32>> = HashSet::new();

        queue.push((0, from));

        let mut dist_so_far = 0;

        while let Some((dist, coord)) = queue.pop() {
            if coord == to {
                return Ok(dist.unsigned_abs());
            }
            if seen_nodes.contains(&coord) {
                continue;
            }
            seen_nodes.insert(coord);

            if dist < dist_so_far {
                dist_so_far = dist;
            }

            // Check all the neighbours, if they're empty and we haven't been there, let's go there
            for vector in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let neighbour = coord + *vector;
                if let Some(node) = self.get_node(&neighbour) {
                    if !seen_nodes.contains(&neighbour) && node == 0 {
                        queue.push((dist - 1, neighbour));
                    }
                }
            }
        }

        Err(error!("No path found!"))
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.nodes {
            for node in row {
                if *node == 0 {
                    write!(f, ".")?;
                } else if *node == 1 {
                    write!(f, "#")?;
                } else {
                    write!(f, "{}", (*node).abs() % 10)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<u32> {
    let coordinates = parse_input(input)?;
    let mut grid = Grid::new(71, 71);

    for coordinate in &coordinates[..1024] {
        grid.drop_byte(coordinate).unwrap();
    }

    grid.get_shortest_distance(
        GridCoordinate { row: 0, column: 0 },
        GridCoordinate {
            row: 70,
            column: 70,
        },
    )
}

fn part2(input: &str) -> Result<String> {
    let coordinates = parse_input(input)?;
    let mut grid = Grid::new(71, 71);

    for coordinate in &coordinates[..1024] {
        grid.drop_byte(coordinate).unwrap();
    }

    // NOTE: I could always try the last known path, which should optimise
    // But.. this runs in 830ms on average, which is under a second, so that's good enough for me
    for coordinate in &coordinates[1024..] {
        grid.drop_byte(coordinate).unwrap();

        if grid
            .get_shortest_distance(
                GridCoordinate { row: 0, column: 0 },
                GridCoordinate {
                    row: 70,
                    column: 70,
                },
            )
            .is_err()
        {
            return Ok(format!("{},{}", coordinate.column, coordinate.row));
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let coordinates = parse_input(TEST_INPUT).unwrap();

        let mut grid = Grid::new(7, 7);

        for coordinate in &coordinates[..12] {
            grid.drop_byte(coordinate).unwrap();
        }

        let dist = grid.get_shortest_distance(
            GridCoordinate { row: 0, column: 0 },
            GridCoordinate { row: 6, column: 6 },
        );

        assert_eq!(dist.unwrap(), 22);
    }

    #[test]
    fn test_part2() {
        let coordinates = parse_input(TEST_INPUT).unwrap();

        let mut grid = Grid::new(7, 7);

        for coordinate in &coordinates[..12] {
            grid.drop_byte(coordinate).unwrap();
        }

        for coordinate in &coordinates[12..] {
            grid.drop_byte(coordinate).unwrap();

            if grid
                .get_shortest_distance(
                    GridCoordinate { row: 0, column: 0 },
                    GridCoordinate { row: 6, column: 6 },
                )
                .is_err()
            {
                assert_eq!(coordinate.column, 6);
                assert_eq!(coordinate.row, 1);
                break;
            }
        }
    }
}
