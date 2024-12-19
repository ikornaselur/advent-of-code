use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fs;

mod parse;

type Pattern = Vec<Colour>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Colour {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug)]
struct PatternTrie {
    children: HashMap<Colour, Box<PatternTrie>>,
    end: bool,
}

impl PatternTrie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            end: false,
        }
    }

    fn insert(&mut self, pattern: &[Colour]) {
        let mut current = self;

        for colour in pattern {
            current = current
                .children
                .entry(colour.clone())
                .or_insert(Box::new(Self::new()));
        }

        current.end = true;
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2024, 19)?
    };

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
    let (towels, patterns) = parse_input(input)?;

    let mut towel_trie = PatternTrie::new();
    for towel in towels {
        towel_trie.insert(&towel);
    }

    let mut valid = 0;
    for pattern in patterns {
        let mut options = vec![&towel_trie];
        for colour in pattern {
            options = options
                .iter()
                .flat_map(|trie| {
                    trie.children
                        .get(&colour)
                        .map(|child| vec![child.as_ref()])
                        .unwrap_or_default()
                })
                .collect();
            // If we reach an end, we wrap around to the start, by just adding `towel_trie` to the
            // options
            // NOTE: If part 2 is about counting options, this might be an issue?
            if options.iter().any(|trie| trie.end) {
                options.push(&towel_trie);
            }
        }
        if options.iter().any(|trie| trie.end) {
            valid += 1;
        }
    }

    Ok(valid)
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
