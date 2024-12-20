use advent::prelude::*;
use parse::parse_heightmap;

mod parse;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Point {
    Start,
    End,
    Height(i32),
}

impl Point {
    /// The elevation of a point
    fn elevation(&self) -> i32 {
        match self {
            Point::Start => 0,
            Point::End => 25,
            Point::Height(h) => *h,
        }
    }
}

struct Heightmap {
    map: Vec<Vec<Point>>,
    steps: Vec<Vec<Option<usize>>>,
    end: GridCoordinate<usize>,
}

impl Heightmap {
    fn new(map: Vec<Vec<Point>>) -> Self {
        let end = map
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter().enumerate().filter_map(move |(y, point)| {
                    if point == &Point::End {
                        Some(GridCoordinate { row: x, column: y })
                    } else {
                        None
                    }
                })
            })
            .next()
            .unwrap();

        let width = map[0].len();
        let height = map.len();
        Self {
            map,
            steps: vec![vec![None; width]; height],
            end,
        }
    }

    /// Find the shortest path from any lowest point to End
    ///
    /// We'll traverse backwards from 'end' so that we can either find the shortest path to
    /// 'Start' or any of the lowest height points.
    fn find_shortest_path_to_end(&mut self, only_start: bool) -> Result<usize> {
        let mut queue = VecDeque::new();
        queue.push_back((self.end, 0));

        while let Some((coord, steps)) = queue.pop_front() {
            match (
                only_start,
                coord.get(&self.map).ok_or(error!("Out of bounds"))?,
            ) {
                (true, &Point::Start) => return Ok(steps),
                (false, p) if p.elevation() == 0 => return Ok(steps),
                _ => {}
            }
            let current_elevation = coord.get(&self.map).unwrap().elevation();

            // Check if we've already been here
            if let Some(prev_steps) = coord.get(&self.steps).ok_or(error!("Out of bounds"))? {
                // We can just continue
                if prev_steps <= &steps {
                    continue;
                }
            }
            coord.set(&mut self.steps, Some(steps)).unwrap();

            // Then we check if we can traverse in any of the four directions
            for next in coord.edge_coordinates(1) {
                if next.within_grid(&self.map) {
                    let node = next.get(&self.map).ok_or(error!("Out of bounds"))?;
                    let next_elevation = node.elevation();
                    if current_elevation - next_elevation <= 1 {
                        queue.push_back((next, steps + 1));
                    }
                }
            }
        }

        panic!("No path found");
    }
}

fn main() -> Result<()> {
    let input = get_input(2022, 12)?;

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
    let mut heightmap = parse_heightmap(input)?;
    heightmap.find_shortest_path_to_end(true)
}

fn part2(input: &str) -> Result<usize> {
    let mut heightmap = parse_heightmap(input)?;
    heightmap.find_shortest_path_to_end(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 29);
    }
}
