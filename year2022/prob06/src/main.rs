use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let input_len = input.len();

    for i in 0..(input_len - 4) {
        // Get a chunk of 4 charcters
        let chars = &input[i..i + 4];

        // Convert to as HashSet
        let set: HashSet<_> = chars.chars().collect();

        // Check if the length is 4, that means the chunk is unique
        if set.len() == 4 {
            return Ok(i + 4);
        }
    }

    Err(error!("No unique chunk found"))
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
