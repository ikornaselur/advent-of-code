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
    width: usize,
    height: usize,
    map: Vec<Vec<Point>>,
    steps: Vec<Vec<Option<usize>>>,
    end: Coordinate<usize>,
}

impl Heightmap {
    fn new(map: Vec<Vec<Point>>) -> Self {
        let end = map
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter().enumerate().filter_map(move |(y, point)| {
                    if point == &Point::End {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .next();

        let width = map[0].len();
        let height = map.len();
        Self {
            width,
            height,
            map,
            steps: vec![vec![None; width]; height],
            end: end.expect("No end point found"),
        }
    }

    /// Find the shortest path from any lowest point to End
    ///
    /// We'll traverse backwards from 'end' so that we can either find the shortest path to
    /// 'Start' or any of the lowest height points.
    fn find_shortest_path_to_end(&mut self, only_start: bool) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((self.end, 0));

        while let Some(((x, y), steps)) = queue.pop_front() {
            match (only_start, self.map[x][y]) {
                (true, Point::Start) => return steps,
                (false, p) if p.elevation() == 0 => return steps,
                _ => {}
            }
            let current_elevation = self.map[x][y].elevation();

            // Check if we've already been here
            if let Some(prev_steps) = self.steps[x][y] {
                // We can just continue
                if prev_steps <= steps {
                    continue;
                }
            }
            self.steps[x][y] = Some(steps);

            // Then we check if we can traverse in any of the four directions
            if x > 0 {
                let left_elevation = self.map[x - 1][y].elevation();
                if current_elevation - left_elevation <= 1 {
                    queue.push_back(((x - 1, y), steps + 1));
                }
            }
            if x < self.height - 1 {
                let right_elevation = self.map[x + 1][y].elevation();
                if current_elevation - right_elevation <= 1 {
                    queue.push_back(((x + 1, y), steps + 1));
                }
            }
            if y > 0 {
                let up_elevation = self.map[x][y - 1].elevation();
                if current_elevation - up_elevation <= 1 {
                    queue.push_back(((x, y - 1), steps + 1));
                }
            }
            if y < self.width - 1 {
                let down_elevation = self.map[x][y + 1].elevation();
                if current_elevation - down_elevation <= 1 {
                    queue.push_back(((x, y + 1), steps + 1));
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
    Ok(heightmap.find_shortest_path_to_end(true))
}

fn part2(input: &str) -> Result<usize> {
    let mut heightmap = parse_heightmap(input)?;
    Ok(heightmap.find_shortest_path_to_end(false))
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
