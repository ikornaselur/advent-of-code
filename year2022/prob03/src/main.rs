use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

/// Split the line in the middle and find which character is in both sides
///
/// Note: There is always exactly just one, so we could search with an iterator from each side..
/// and do something with that, right?
///
/// But, let's just find intersection of sets of chars
fn find_dup(line: &str) -> char {
    let line_len = line.len();
    let left = &line[..line_len / 2];
    let right = &line[line_len / 2..];

    let left_chars: HashSet<_> = left.chars().collect();
    let right_chars: HashSet<_> = right.chars().collect();

    let intersection: HashSet<_> = left_chars.intersection(&right_chars).cloned().collect();

    intersection.iter().next().cloned().unwrap()
}

/// Return the score of the char
///
/// The score is 1 to 26 for a to z and 27 to 52 for A to Z
fn char_score(char: char) -> u32 {
    if char.is_lowercase() {
        char as u32 - 'a' as u32 + 1
    } else {
        char as u32 - 'A' as u32 + 27
    }
}

fn part1(input: &str) -> Result<u32> {
    Ok(input.lines().fold(0, |acc, line| {
        acc + char_score(find_dup(line))
    }))
}

fn part2(input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_find_dup() {
        assert_eq!(find_dup("abcdea"), 'a');
        assert_eq!(find_dup("abcdeb"), 'b');
    }

    #[test]
    fn test_char_score() {
        assert_eq!(char_score('a'), 1);
        assert_eq!(char_score('z'), 26);
        assert_eq!(char_score('A'), 27);
        assert_eq!(char_score('Z'), 52);
    }
}
