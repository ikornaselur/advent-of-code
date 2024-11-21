use advent::prelude::*;
use cpu::CPU;
use parse::parse_instructions;

mod cpu;
mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

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

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
