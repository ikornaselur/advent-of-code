use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fmt;
use std::fs;

mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Wall,
    Floor,
    Path(usize), // Track the path from start to end, with the distance to end
    Start,
    End,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Node::Wall => '#',
            Node::Floor => '.',
            Node::Path(_) => '*',
            Node::Start => 'S',
            Node::End => 'E',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    nodes: Vec<Vec<Node>>,
    start: GridCoordinate<i32>,
    end: GridCoordinate<i32>,
    current: GridCoordinate<i32>,
    cheating: Option<GridCoordinate<i32>>,
    //path: HashSet<GridCoordinate<i32>>,
}

impl Map {
    fn new(nodes: Vec<Vec<Node>>) -> Self {
        let (start, end) = nodes.iter().enumerate().fold(
            (GridCoordinate::new(0, 0), GridCoordinate::new(0, 0)),
            |(start, end), (y, row)| {
                row.iter()
                    .enumerate()
                    .fold((start, end), |(start, end), (x, node)| match node {
                        Node::Start => (GridCoordinate::new(y as i32, x as i32), end),
                        Node::End => (start, GridCoordinate::new(y as i32, x as i32)),
                        _ => (start, end),
                    })
            },
        );
        // We'll overwrite the start and end with just 'floor'
        let mut nodes = nodes;
        nodes[start.row as usize][start.column as usize] = Node::Floor;
        nodes[end.row as usize][end.column as usize] = Node::Floor;
        Self {
            nodes,
            start,
            end,
            current: start,
            cheating: None,
            //path: HashSet::new(),
        }
    }

    fn get_node(&self, coordinate: GridCoordinate<i32>) -> Option<Node> {
        if coordinate.within_grid(&self.nodes) {
            Some(self.nodes[coordinate.row as usize][coordinate.column as usize])
        } else {
            None
        }
    }

    /// Solve the maze from start to end
    ///
    /// There should be exactly one path through the maze, which we'll mark with the distance from
    /// the start, so that we can easily calculate savings when we start cheating
    fn solve_maze(&mut self, from: GridCoordinate<i32>, dist: usize) {
        for next in from.edge_coordinates(1) {
            if let Some(Node::Floor) = self.get_node(next) {
                self.nodes[next.row as usize][next.column as usize] = Node::Path(dist + 1);
                self.solve_maze(next, dist + 1);
            }
        }
    }

    /// We search for cheats by just checking how many nodes are within a manhattan distance of
    /// <dist>, then check if the value of those nodes is more than <dist> higher than the current
    /// node. That means cheating saves time.
    fn find_cheats(&self, dist: i32) -> Vec<usize> {
        let mut cheats = Vec::new();

        for row in 0..(self.nodes.len() as i32) {
            for column in 0..(self.nodes[0].len() as i32) {
                let current = GridCoordinate { row, column };
                if let Some(Node::Path(current_dist)) = self.get_node(current) {
                    for option in current.surrounding_coordinates(dist as usize) {
                        let option_dist = option.manhattan_distance(&current);
                        if let Some(Node::Path(cheating_dist)) = self.get_node(option) {
                            let time_saved =
                                cheating_dist as i32 - current_dist as i32 - option_dist;
                            if time_saved > 1 {
                                cheats.push(time_saved as usize);
                            }
                        };
                    }
                }
            }
        }
        cheats
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.nodes.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                match node {
                    _ if self.current.row == y as i32 && self.current.column == x as i32 => {
                        write!(f, "X")?
                    }
                    _ if self.cheating.is_some()
                        && self.cheating.unwrap().row == y as i32
                        && self.cheating.unwrap().column == x as i32 =>
                    {
                        write!(f, "C")?
                    }
                    Node::Path(_) => write!(f, " ")?,
                    node => write!(f, "{}", node)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2024, 20)?
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
    let mut map = parse_input(input)?;

    map.solve_maze(map.start, 0);

    let savings = map.find_cheats(2);

    Ok(savings.iter().filter(|c| **c >= 100).count())
}

fn part2(input: &str) -> Result<usize> {
    let mut map = parse_input(input)?;

    map.solve_maze(map.start, 0);

    let savings = map.find_cheats(20);

    Ok(savings.iter().filter(|c| **c >= 100).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
