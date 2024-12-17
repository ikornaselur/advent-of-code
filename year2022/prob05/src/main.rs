use advent::prelude::*;

mod parse;

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn main() -> Result<()> {
    let input = get_input(2022, 5)?;

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

fn part1(input: &str) -> Result<String> {
    let (mut stacks, instructions) = parse::parse_input(input)?;

    // Each instruction tells us how many crates to move from one stack to another, so an
    // instruction 'move 1 from 2 to 1' means "move 1 crate from stack 2 onto stack 1"
    // Crates are moved *one at a time*, meaning that if an instruction says 'move 2 from 3 to 1'
    // it means to move the top crate from stack 3 to stack 1, then move the next top crate.
    for instruction in instructions {
        let from_idx = instruction.from - 1;
        let to_idx = instruction.to - 1;

        for _ in 0..instruction.count {
            let crate_ = stacks[from_idx].crates.pop().unwrap();
            stacks[to_idx].crates.push(crate_);
        }
    }

    // Finally we go through the stacks and construct a string of the top element of each
    Ok(stacks
        .iter()
        .map(|stack| stack.crates.last().unwrap())
        .collect::<String>())
}

fn part2(input: &str) -> Result<String> {
    let (mut stacks, instructions) = parse::parse_input(input)?;

    // Each instruction tells us how many crates to move from one stack to another, so an
    // instruction 'move 1 from 2 to 1' means "move 1 crate from stack 2 onto stack 1"
    // Crates are all moved at once this time.
    for instruction in instructions {
        let from_idx = instruction.from - 1;
        let to_idx = instruction.to - 1;

        let split_off_index = stacks[from_idx].crates.len() - instruction.count;
        let tail = stacks[from_idx].crates.split_off(split_off_index);
        stacks[to_idx].crates.extend(tail);
    }

    // Finally we go through the stacks and construct a string of the top element of each
    Ok(stacks
        .iter()
        .map(|stack| stack.crates.last().unwrap())
        .collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), String::from("CMZ"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), String::from("MCD"));
    }
}
