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

    program: Vec<u64>,
    output: Vec<u64>,
}

impl CPU {
    fn new(a: u64, b: u64, c: u64, program: Vec<u64>) -> Self {
        Self {
            a,
            b,
            c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn fetch_opcode(&mut self) -> Result<u64> {
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

        Ok(value)
    }

    fn fetch_combo_operand(&mut self) -> Result<u64> {
        let value = self
            .program
            .get(self.ip)
            .copied()
            .ok_or(error!("Invalid instruction pointer"))?;

        self.ip += 1;

        match value {
            0..=3 => Ok(value),
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
            3 => self.jnz()?,
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

/// How many values match, in a row, from the back
///
/// For example, [1,2,3,4,5] and [3,4,5] would return 3
/// If the second value is a full match, we return a 'full' flag
fn reverse_match_count(a: &[u64], b: &[u64]) -> (usize, bool) {
    let rev_match_count = a
        .iter()
        .rev()
        .zip(b.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();

    (rev_match_count, rev_match_count == b.len())
}

fn part2(input: &str) -> Result<u64> {
    let cpu = parse_input(input)?;

    // By just playing around with values, I've noticed the following:
    //
    // 1. The output length seems to be the power of the initial A
    //
    // That is:
    //   - When A is '1', the output length is 1
    //   - When A is '10', the output length is 2
    //   - When A is '100', the output length is 3
    //   And so on.. though it's not exact, it gives an idea of the range where A will be, it's
    //   somehwere around 10^(program.len()) to 10^(program.len() + 1) ish
    //
    //  With the input program length of 16, we're looking around values that look like
    //  100_000_000_000_000 - So brute forcing is pretty much out of the question
    //
    //
    // 2. The number of steps the program takes increases with increasing A, which I guess makes
    //    sense. The final opcodes in the program are `3, 3, 0`, so it's jumping to the start while
    //    A has values greater than 0
    //
    // With the values mentioned above, it seems the program takes ~128 steps to run to completion
    //
    // 3. Given that we keep jumping to the start and repeat the program essentially, can we figure
    //    out a much lower A that will have the same *end* output? Then work from there?
    //    Are there patterns that loop?
    //
    // 4. Looking at the ends of the programs and outputs, the overlap seems to grow at a rate of
    //    8, that is, a new number will match when a is multiplied with 8
    //
    //
    // At this point.. trying to multiply a by 8 each time we have a match, seems to do the job!

    let mut a = 0;
    let mut max_end_match = 0;

    loop {
        let mut cpu = cpu.clone();
        cpu.a = a;

        while cpu.execute_step().is_ok() {}

        let (rev_match_count, full_match) = reverse_match_count(&cpu.output, &cpu.program);
        if full_match {
            return Ok(a);
        } else if !full_match && rev_match_count > max_end_match {
            max_end_match = rev_match_count;

            a *= 8;
        } else {
            a += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const TEST2_INPUT: &str = include_str!("../test2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(TEST_INPUT).unwrap(),
            String::from("4,6,3,5,6,3,5,2,1,0")
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST2_INPUT).unwrap(), 117_440);
    }
}
