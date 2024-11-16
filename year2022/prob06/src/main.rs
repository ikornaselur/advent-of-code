use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn get_marker(input: &str, chunk_size: usize) -> Result<usize> {
    let mut set = HashSet::new();
    (0..=input.len() - chunk_size)
        .find(|&i| {
            set.clear();
            set.extend(input[i..i + chunk_size].chars());
            set.len() == chunk_size
        })
        .map(|i| i + chunk_size)
        .ok_or_else(|| error!("No marker found"))
}

fn part1(input: &str) -> Result<usize> {
    get_marker(input, 4)
}

fn part2(input: &str) -> Result<usize> {
    get_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 29);
    }
}
