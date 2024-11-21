use advent::prelude::*;
use cpu::{Instruction, CPU};
use itertools::Itertools;
use parse::parse_instructions;

mod cpu;
mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!("{}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let mut cpu = CPU::new();
    let instructions = parse_instructions(input)?;

    let mut signal_strengths = 0;
    let mut signal_check = 20;

    for instruction in instructions {
        // We execute instructions, making note of the 'a' register value at different intervals as
        // specified in the problem (at the 20th cycle, then every 40th after that)
        if cpu.cycles + instruction.cycles() >= signal_check {
            signal_strengths += (signal_check as i32) * cpu.reg_a;
            signal_check += 40;
        }

        cpu.run_instruction(instruction);
    }

    Ok(signal_strengths)
}

fn part2(input: &str) -> Result<String> {
    let mut cpu = CPU::new();
    let instructions = parse_instructions(input)?;

    let mut screen = ['.'; 240];

    for instruction in instructions {
        // Before we run the instruction, we check if the sprite is visible during the cycles we
        // will run, as the sprite can only move _after_ we execute the instruction
        // We also know that we only have two instructions, one which is 1 cycle (noop) and one
        // which is 2 cycles (addx), so we can just check the current spot and the next only if
        // it's an addx instruction
        let sprite_range = cpu.sprite_range();
        let current_cycle = cpu.cycles;
        if sprite_range.contains(&((current_cycle % 40) as i32)) {
            screen[current_cycle] = '#';
        }
        if let Instruction::AddX(_) = instruction {
            if sprite_range.contains(&(((current_cycle + 1) % 40) as i32)) {
                screen[current_cycle + 1] = '#';
            }
        }

        cpu.run_instruction(instruction);
    }

    // Return the 240 chars as 6 lines of 40 chars each, in a single string with newlines
    Ok(screen
        .iter()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 13140);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT).unwrap();
        let expected = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ]
        .join("\n");
        assert_eq!(result, expected);
    }
}
