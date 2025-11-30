use advent::prelude::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = get_input(2022, 3)?;

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
    Ok(input
        .lines()
        .fold(0, |acc, line| acc + char_score(find_dup(line))))
}

/// Find the common character between all lines in a chunk and return the score of it
///
/// Will panic if there is not exactly one character common
fn chunk_score(chunk: &[&str]) -> u32 {
    let intersection = chunk
        .iter()
        .fold(None, |acc: Option<HashSet<char>>, line| {
            if let Some(acc) = acc {
                let chars: HashSet<_> = line.chars().collect();
                Some(acc.intersection(&chars).cloned().collect())
            } else {
                Some(line.chars().collect::<HashSet<_>>())
            }
        })
        .unwrap();

    if intersection.len() != 1 {
        panic!("Invalid common chars");
    }

    let common_char = intersection.iter().next().cloned().unwrap();
    char_score(common_char)
}

fn part2(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk_score(&chunk.collect::<Vec<_>>()))
        .sum())
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 70);
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
