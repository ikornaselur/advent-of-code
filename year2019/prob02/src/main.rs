use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn run_program(memory: &mut [usize]) -> Result<()> {
    let mut idx = 0;

    while let Some(val) = &memory.get(idx) {
        match val {
            1 => {
                let a = *memory.get(idx + 1).unwrap();
                let b = *memory.get(idx + 2).unwrap();
                let dest = *memory.get(idx + 3).unwrap();
                memory[dest] = memory[a] + memory[b];
                idx += 4;
            }
            2 => {
                let a = *memory.get(idx + 1).unwrap();
                let b = *memory.get(idx + 2).unwrap();
                let dest = *memory.get(idx + 3).unwrap();
                memory[dest] = memory[a] * memory[b];
                idx += 4;
            }
            99 => {
                break;
            }
            _ => return Err(error!("Invalid opcode")),
        }
    }

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut memory = parse_input(input)?;

    memory[1] = 12;
    memory[2] = 2;

    run_program(&mut memory)?;

    Ok(memory[0])
}

fn part2(input: &str) -> Result<usize> {
    let memory = parse_input(input)?;
    let target = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = memory.clone();
            mem[1] = noun;
            mem[2] = verb;
            if run_program(&mut mem).is_ok() && mem[0] == target {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(error!("No solution found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        // The test example doesn't involve changing the early digits.. so we skip that
        let mut digits = parse_input(TEST_INPUT).unwrap();
        run_program(&mut digits).unwrap();

        assert_eq!(digits[0], 3500);
    }
}
