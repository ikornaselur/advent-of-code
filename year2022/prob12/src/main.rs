use advent::prelude::*;
use parse::parse_heightmap;

mod parse;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Clone)]
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
    start: Coordinate<usize>,
    end: Coordinate<usize>,
}

impl Heightmap {
    fn new(map: Vec<Vec<Point>>) -> Self {
        let mut start = None;
        let mut end = None;
        for (x, row) in map.iter().enumerate() {
            for (y, point) in row.iter().enumerate() {
                match point {
                    Point::Start => start = Some((x, y)),
                    Point::End => end = Some((x, y)),
                    _ => {}
                }
            }
        }
        let width = map[0].len();
        let height = map.len();
        Self {
            width,
            height,
            map,
            steps: vec![vec![None; width]; height],
            start: start.expect("No start point found"),
            end: end.expect("No end point found"),
        }
    }

    /// Find the shortest path from Start to End
    ///
    /// We do this by creating a queue of valid steps to take, making note of how many steps we've
    /// taken to get there
    /// If we've been there in fewer steps, we stop traversing
    fn find_shortest_path(&mut self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));

        while let Some(((x, y), steps)) = queue.pop_front() {
            if let Some(end_steps) = self.steps[self.end.0][self.end.1] {
                if steps > end_steps {
                    // Can we just return here immediately?
                    continue;
                }
            }
            // Check if we've already been here
            if let Some(prev_steps) = self.steps[x][y] {
                // We can just continue
                if prev_steps <= steps {
                    continue;
                }
                todo!("Not implemented. Do we need to?");
            }
            self.steps[x][y] = Some(steps);

            let current_elevation = self.map[x][y].elevation();

            // Then we check if we can traverse in any of the four directions
            if x > 0 {
                let left_elevation = self.map[x - 1][y].elevation();
                if left_elevation - current_elevation <= 1 {
                    queue.push_back(((x - 1, y), steps + 1));
                }
            }
            if x < self.height - 1 {
                let right_elevation = self.map[x + 1][y].elevation();
                if right_elevation - current_elevation <= 1 {
                    queue.push_back(((x + 1, y), steps + 1));
                }
            }
            if y > 0 {
                let up_elevation = self.map[x][y - 1].elevation();
                if up_elevation - current_elevation <= 1 {
                    queue.push_back(((x, y - 1), steps + 1));
                }
            }
            if y < self.width - 1 {
                let down_elevation = self.map[x][y + 1].elevation();
                if down_elevation - current_elevation <= 1 {
                    queue.push_back(((x, y + 1), steps + 1));
                }
            }
        }

        self.steps[self.end.0][self.end.1].unwrap()
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
    let mut heightmap = parse_heightmap(input)?;
    Ok(heightmap.find_shortest_path())
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
