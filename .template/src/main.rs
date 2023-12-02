const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("## Part 1");
    println!(" > {}", part1(INPUT));

    println!("## Part 2");
    println!(" > {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT), 0);
    }
}
