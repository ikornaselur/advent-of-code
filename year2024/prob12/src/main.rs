use advent::prelude::*;
use parse::parse_input;

mod parse;

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

/// Take a coordinate and return a hashset of all the points in this lot
fn map_out_plot(plot_map: &[Vec<char>], pos: Coordinate<usize>) -> HashSet<Coordinate<usize>> {
    let plot_id = plot_map[pos.0][pos.1];
    let mut plot: HashSet<Coordinate<usize>> = HashSet::new();

    let mut queue: VecDeque<Coordinate<usize>> = VecDeque::from(vec![pos]);

    while let Some((y, x)) = queue.pop_front() {
        if plot.contains(&(y, x)) {
            // No need to check this one again
            // panic!("Shouldn't need this");
            continue;
        }

        if plot_map[y][x] != plot_id {
            // Not this plot
            continue;
        }

        plot.insert((y, x));

        if y > 0 {
            queue.push_back((y - 1, x));
        }
        if y < plot_map.len() - 1 {
            queue.push_back((y + 1, x));
        }
        if x > 0 {
            queue.push_back((y, x - 1));
        }
        if x < plot_map[0].len() - 1 {
            queue.push_back((y, x + 1));
        }
    }

    plot
}

fn count_plot_edges(plot: &HashSet<Coordinate<usize>>, plot_map: &[Vec<char>]) -> usize {
    let mut edges = 0;
    for (y, x) in plot {
        if *y > 0 && !plot.contains(&(*y - 1, *x)) {
            edges += 1;
        }
        if *y == 0 {
            edges += 1;
        }
        if *y < plot_map.len() - 1 && !plot.contains(&(*y + 1, *x)) {
            edges += 1;
        }
        if *y == plot_map.len() - 1 {
            edges += 1;
        }
        if *x > 0 && !plot.contains(&(*y, *x - 1)) {
            edges += 1;
        }
        if *x == 0 {
            edges += 1;
        }
        if *x < plot_map[0].len() - 1 && !plot.contains(&(*y, *x + 1)) {
            edges += 1;
        }
        if *x == plot_map[0].len() - 1 {
            edges += 1;
        }
    }
    edges
}

fn part1(input: &str) -> Result<usize> {
    let plot_map = parse_input(input)?;

    // Go through the characters in the plots, growing out each plot as we find it
    let mut plots: Vec<HashSet<Coordinate<usize>>> = Vec::new();
    let mut seen_spots: HashSet<Coordinate<usize>> = HashSet::new();

    for y in 0..plot_map.len() {
        for x in 0..plot_map[0].len() {
            if seen_spots.contains(&(y, x)) {
                continue;
            }

            let plot = map_out_plot(&plot_map, (y, x));
            seen_spots.extend(&plot);
            plots.push(plot);
        }
    }

    Ok(plots.iter().fold(0, |acc, plot| {
        acc + plot.len() * count_plot_edges(plot, &plot_map)
    }))
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const TEST_INPUT2: &str = include_str!("../test2.txt");

    #[test]
    fn test_part1_test1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 140);
    }

    #[test]
    fn test_part1_test2() {
        assert_eq!(part1(TEST_INPUT2).unwrap(), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_map_out_plot() {
        let plot_map = vec![vec!['A', 'A'], vec!['B', 'A']];

        let plot = map_out_plot(&plot_map, (0, 0));

        assert_eq!(plot.len(), 3);
        assert!(plot.contains(&(0, 0)));
        assert!(plot.contains(&(0, 1)));
        assert!(plot.contains(&(1, 1)));
    }
}
