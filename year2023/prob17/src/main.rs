use advent::prelude::*;

struct Layout {
    nodes: Vec<Vec<u8>>,
}

type HeatLoss = i32;
type StraightDistance = i32;
type Key = (
    HeatLoss,
    GridCoordinate<usize>,
    CompassDirection,
    StraightDistance,
);

/// Notes
///
/// * Since we have a limit of straight lines, we can't just do a basic flood algorithm (is that
///   what it's called?)
/// * We can also easily have loops, since we can turn any way (except backwards)
/// * Might be worth having as core for each path we are looking at, which is not just the current
///   cost but also have a penalty for just how far we are from the goal? This way we don't waste
///   time on paths that are really far away
///     - When we find a path this way, we will have to continue processing all other paths that
///       are still in the queue, but we can exit as soon as they go over the best known cost
impl Layout {
    /// Find the cheapest path from one coordinate to another
    ///
    /// There is a limitation of how long a straight line can be, meaning that if the current path
    /// has taken a staright line for <max_straight_line> nodes in a row, it _has_ to take a turn
    /// next
    ///
    /// The start node does not incurr a cost, unless the path takes us back over that node
    fn find_cheapest_path(
        &self,
        to: GridCoordinate<usize>,
        min_straight_line: i32,
        max_straight_line: i32,
    ) -> Result<i32> {
        let mut seen: HashSet<(GridCoordinate<usize>, CompassDirection, StraightDistance)> =
            HashSet::new();

        // This is a max heap, so we store the scores negative to turn it into a min heap
        let mut heap: BinaryHeap<Key> = BinaryHeap::new();

        // The heap is a tuple of:
        //
        //   * The total heat loss so far at this node (stored negative to turn max-heap into min-heap)
        //   * The coordinate of the current node
        //   * The direction we are heading
        //   * How many nodes we have gone straight in a row - if we just turned, it's 0, this
        //     includes in the start
        //
        // We start by setting the heap to the two first steps having been taken, basically, since
        // we start in the upper right corner at (0, 0) we can either so South of East. Since we
        // don't count the heat from the start node, the initial heat will be the first node we
        // step on
        heap.push((
            -self.node_heat(GridCoordinate { row: 1, column: 0 }),
            GridCoordinate { row: 1, column: 0 },
            CompassDirection::South,
            0,
        ));
        heap.push((
            -self.node_heat(GridCoordinate { row: 0, column: 1 }),
            GridCoordinate { row: 0, column: 1 },
            CompassDirection::East,
            0,
        ));

        while !heap.is_empty() {
            let (heat_loss, coord, direction, straight_distance) =
                heap.pop().ok_or(error!("No more nodes in heap"))?;

            // Work with heat loss as positive
            let heat_loss = -heat_loss;

            if coord == to {
                return Ok(heat_loss);
            }

            if !seen.insert((coord, direction, straight_distance)) {
                // We had already seen this node, so we can skip it
                continue;
            }

            // Continue the path in all directions, except for:
            //  * Backwards
            //  * Forwards if we exceed the max distance
            //  * If we go off the grid
            for next_direction in &[
                CompassDirection::North,
                CompassDirection::South,
                CompassDirection::East,
                CompassDirection::West,
            ] {
                // Can't go backwards
                if next_direction == &direction.opposite() {
                    continue;
                }

                // If forwards, we can only continue if we don't exceed max distance
                if next_direction == &direction {
                    if straight_distance < (max_straight_line - 1) {
                        if let Some(next_coord) = coord.shifted(direction) {
                            if next_coord.within_grid(&self.nodes) {
                                let next_node_heat = self.node_heat(next_coord);
                                heap.push((
                                    -(heat_loss + next_node_heat),
                                    next_coord,
                                    direction,
                                    straight_distance + 1,
                                ));
                            }
                        }
                    }
                    continue;
                }
                if straight_distance >= min_straight_line {
                    // Try to turn
                    if let Some(next_coord) = coord.shifted(*next_direction) {
                        if next_coord.within_grid(&self.nodes) {
                            heap.push((
                                -(heat_loss + self.node_heat(next_coord)),
                                next_coord,
                                *next_direction,
                                0,
                            ));
                        }
                    }
                }
            }
        }
        Err(error!("No path found"))
    }

    fn node_heat(&self, coord: GridCoordinate<usize>) -> i32 {
        (*coord.get(&self.nodes).unwrap()) as i32
    }
}

impl FromStr for Layout {
    type Err = AdventError;

    // Parse lines of digits into u8
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .map(|l| l.chars().map(|c| (c as u8) - 48).collect())
            .collect();

        Ok(Layout { nodes })
    }
}

fn main() -> Result<()> {
    let input = get_input(2023, 17)?;

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

fn part1(input: &str) -> Result<i32> {
    let layout: Layout = input.parse()?;

    layout.find_cheapest_path(
        GridCoordinate {
            row: layout.nodes.len() - 1,
            column: layout.nodes[0].len() - 1,
        },
        0,
        3,
    )
}

fn part2(input: &str) -> Result<i32> {
    let layout: Layout = input.parse()?;

    layout.find_cheapest_path(
        GridCoordinate {
            row: layout.nodes.len() - 1,
            column: layout.nodes[0].len() - 1,
        },
        3,
        10,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 94);
    }

    #[test]
    fn test_layout_from_str() {
        let layout: Layout = "123\n890".parse().unwrap();

        assert_eq!(layout.nodes, vec![vec![1, 2, 3], vec![8, 9, 0]]);
    }

    #[test]
    fn test_layout_find_cheapest_path_no_min_straight_line() {
        let layout: Layout = "1456\n1416\n1816\n1111".parse().unwrap();

        assert_eq!(
            layout
                .find_cheapest_path(GridCoordinate { row: 3, column: 3 }, 0, 4)
                .unwrap(),
            6
        );
        assert_eq!(
            layout
                .find_cheapest_path(GridCoordinate { row: 3, column: 3 }, 0, 3)
                .unwrap(),
            6
        );
        assert_eq!(
            layout
                .find_cheapest_path(GridCoordinate { row: 3, column: 3 }, 0, 2)
                .unwrap(),
            9
        );
        assert_eq!(
            layout
                .find_cheapest_path(GridCoordinate { row: 3, column: 3 }, 0, 1)
                .unwrap(),
            16
        );
    }

    #[test]
    fn test_layout_node_heat() {
        let layout: Layout = "1456\n1416\n1816\n1111".parse().unwrap();

        assert_eq!(layout.node_heat(GridCoordinate { row: 0, column: 0 }), 1);
        assert_eq!(layout.node_heat(GridCoordinate { row: 0, column: 1 }), 4);
        assert_eq!(layout.node_heat(GridCoordinate { row: 1, column: 0 }), 1);
        assert_eq!(layout.node_heat(GridCoordinate { row: 2, column: 1 }), 8);
    }
}
