use advent::prelude::*;
use parse::parse_input;
use std::fmt;

mod parse;

const MOVE_COST: i32 = 1;
const TURN_COST: i32 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Wall,
    Floor,
    Start,
    End,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Node::Wall => '#',
            Node::Floor => '.',
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
    direction: CompassDirection,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.nodes.iter() {
            for node in row.iter() {
                write!(f, "{}", node)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(nodes: Vec<Vec<Node>>) -> Self {
        // NOTE: Is this too much? It'll also iterate over all nodes, even when start + end have
        // been found.. but it's fine for AoC
        let (start, end) = nodes.iter().enumerate().fold(
            (GridCoordinate::new(0, 0), GridCoordinate::new(0, 0)),
            |(start, end), (y, row)| {
                row.iter()
                    .enumerate()
                    .fold((start, end), |(start, end), (x, node)| match node {
                        Node::Start => (GridCoordinate::new(x as i32, y as i32), end),
                        Node::End => (start, GridCoordinate::new(x as i32, y as i32)),
                        _ => (start, end),
                    })
            },
        );
        Self {
            nodes,
            start,
            end,
            direction: CompassDirection::East,
        }
    }
}

fn solve_maze(map: &Map) -> u32 {
    // Let's do a BFS with a queue, trying lowest scores first
    // Since BinaryHeap is a max-heap, we'll store the score as negative numbers!
    let mut queue: BinaryHeap<(i32, GridCoordinate<i32>, CompassDirection)> =
        BinaryHeap::from(vec![(0, map.start, map.direction)]);
    // We'll keep track of seen nodes _with_ the direction, as turning around is expensive, but is
    // a 'different state' for us to try
    let mut seen_nodes: HashSet<(GridCoordinate<i32>, CompassDirection)> = HashSet::new();
    let mut min_score = None;

    while let Some((score, coord, direction)) = queue.pop() {
        if seen_nodes.contains(&(coord, direction)) {
            continue;
        }
        seen_nodes.insert((coord, direction));

        if coord == map.end {
            let score = score.unsigned_abs();
            if let Some(min) = min_score {
                if score < min {
                    min_score = Some(score);
                }
            } else {
                min_score = Some(score);
            }
            continue;
        }

        // If we already have min_score and have a higher current score, we prune
        if let Some(min_score) = min_score {
            if score.unsigned_abs() >= min_score {
                continue;
            }
        }

        // If there's not a wall in front of us, let's queue that up
        let forward = coord + direction.as_vector();
        if let Some(node) = forward.get(&map.nodes) {
            if *node != Node::Wall {
                queue.push((score - MOVE_COST, forward, direction));
            }
        }
        // Let's also queue up turning left or right, if there's an opening
        let left = coord + direction.left_90().as_vector();
        if let Some(node) = left.get(&map.nodes) {
            if *node != Node::Wall {
                queue.push((score - TURN_COST, coord, direction.left_90()));
            }
        }

        let right = coord + direction.right_90().as_vector();
        if let Some(node) = right.get(&map.nodes) {
            if *node != Node::Wall {
                queue.push((score - TURN_COST, coord, direction.right_90()));
            }
        }
    }

    min_score.unwrap()
}

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, INPUT)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, INPUT)?;
    println!(" > {}", result);

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

fn part1(input: &str) -> Result<u32> {
    let map = parse_input(input)?;

    // println!("{}", map);

    let result = solve_maze(&map);

    Ok(result)
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const TEST2_INPUT: &str = include_str!("../test2.txt");

    #[test]
    fn test_part1_test1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 7036);
    }

    #[test]
    fn test_part1_test2() {
        assert_eq!(part1(TEST2_INPUT).unwrap(), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
