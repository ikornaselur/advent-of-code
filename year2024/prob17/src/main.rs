use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct CPU {
    a: u64,
    b: u64,
    c: u64,

    ip: usize,

    program: Vec<u8>,
    output: Vec<u64>,
}

impl CPU {
    fn new(a: u64, b: u64, c: u64, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn fetch_opcode(&mut self) -> Result<u8> {
        let value = self
            .program
            .get(self.ip)
            .copied()
            .ok_or(error!("Invalid instruction pointer"))?;

        self.ip += 1;

        Ok(value)
    }

    fn fetch_literal_operand(&mut self) -> Result<u64> {
        let value = self
            .program
            .get(self.ip)
            .copied()
            .ok_or(error!("Invalid instruction pointer"))?;

        self.ip += 1;

        Ok(value as u64)
    }

    fn fetch_combo_operand(&mut self) -> Result<u64> {
        let value = self
            .program
            .get(self.ip)
            .copied()
            .ok_or(error!("Invalid instruction pointer"))?;

        self.ip += 1;

        match value {
            0..=3 => Ok(value as u64),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            7 => panic!("7 is reserved"),
            _ => unreachable!(),
        }
    }

    fn execute_step(&mut self) -> Result<()> {
        let opcode = self.fetch_opcode()?;

        match opcode {
            0 => self.adv()?,
            1 => self.bxl()?,
            2 => self.bst()?,
            3 => self.jnz()?, // TODO: Verify?
            4 => self.bxc()?,
            5 => self.out()?,
            6 => self.bdv()?,
            7 => self.cdv()?,
            _ => unreachable!(),
        }

        Ok(())
    }

    fn adv(&mut self) -> Result<()> {
        let denominator = 2u64.pow(self.fetch_combo_operand()? as u32);
        self.a /= denominator;
        Ok(())
    }

    fn bxl(&mut self) -> Result<()> {
        let value = self.fetch_literal_operand()?;
        self.b ^= value;
        Ok(())
    }

    fn bst(&mut self) -> Result<()> {
        let value = self.fetch_combo_operand()?;
        self.b = value % 8;
        Ok(())
    }

    fn jnz(&mut self) -> Result<()> {
        if self.a > 0 {
            self.ip = self.fetch_literal_operand()? as usize;
        }
        Ok(())
    }

    fn bxc(&mut self) -> Result<()> {
        self.b ^= self.c;
        self.ip += 1; // Skips instruction, for legacy reasons
        Ok(())
    }

    fn out(&mut self) -> Result<()> {
        let value = self.fetch_combo_operand()? % 8;
        self.output.push(value);
        Ok(())
    }

    fn bdv(&mut self) -> Result<()> {
        let denominator = 2u64.pow(self.fetch_combo_operand()? as u32);
        self.b = self.a / denominator;
        Ok(())
    }

    fn cdv(&mut self) -> Result<()> {
        let denominator = 2u64.pow(self.fetch_combo_operand()? as u32);
        self.c = self.a / denominator;
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, INPUT)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, INPUT)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        INPUT,
    );

    Ok(())
}

fn part1(input: &str) -> Result<String> {
    let mut cpu = parse_input(input)?;

    while cpu.execute_step().is_ok() {}

    let output: String = cpu
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Ok(output)
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
        assert_eq!(
            part1(TEST_INPUT).unwrap(),
            String::from("4,6,3,5,6,3,5,2,1,0")
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
