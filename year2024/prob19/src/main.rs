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

impl Colour {
    fn to_char(&self) -> char {
        match self {
            Colour::White => 'w',
            Colour::Blue => 'u',
            Colour::Black => 'b',
            Colour::Red => 'r',
            Colour::Green => 'g',
        }
    }
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

    /// How many ways can a pattern be constructed?
    fn options_count(&self, pattern: &[Colour], cache: &mut HashMap<String, usize>) -> usize {
        // Check if we've seen this pattern before
        let key = pattern.iter().map(Colour::to_char).collect::<String>();
        if let Some(count) = cache.get(&key) {
            return *count;
        }

        let mut count = 0;
        let mut options = vec![self];
        for (idx, colour) in pattern.iter().enumerate() {
            options = options
                .iter()
                .flat_map(|trie| {
                    trie.children
                        .get(colour)
                        .map(|child| vec![child.as_ref()])
                        .unwrap_or_default()
                })
                .collect();

            // If we reach the end, we'll have to check how many ways we can construct the rest..
            // we'll just recurse here
            // I think we'll need to multiply by the number of ways we got here.. right? Because
            // that's all options to _get here_
            let ends_count = options.iter().filter(|trie| trie.end).count();
            let sub_pattern = &pattern[idx + 1..];
            if !sub_pattern.is_empty() {
                count += self.options_count(sub_pattern, cache) * ends_count;
            }
        }

        let total_count = count + options.iter().filter(|trie| trie.end).count();
        cache.insert(key, total_count);
        total_count
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
    let mut cache: HashMap<String, usize> = HashMap::new();

    Ok(patterns.iter().fold(0, |acc, pattern| {
        if towel_trie.options_count(pattern, &mut cache) > 0 {
            acc + 1
        } else {
            acc
        }
    }))
}

fn part2(input: &str) -> Result<usize> {
    let (towels, patterns) = parse_input(input)?;

    let mut towel_trie = PatternTrie::new();
    for towel in towels {
        towel_trie.insert(&towel);
    }
    let mut cache: HashMap<String, usize> = HashMap::new();

    Ok(patterns.iter().fold(0, |acc, pattern| {
        acc + towel_trie.options_count(pattern, &mut cache)
    }))
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 16);
    }
}
