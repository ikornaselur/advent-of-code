use crate::CPU;
use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_register(input: &str) -> IResult<&str, (char, u64)> {
    separated_pair(
        preceded(tag("Register "), one_of("ABC")),
        tag(": "),
        nom_unsigned_digit,
    )(input)
}

fn nom_program(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(
        tag("Program: "),
        separated_list1(char(','), nom_unsigned_digit),
    )(input)
}

pub fn parse_input(input: &str) -> Result<CPU> {
    let (_, (registers, program)) = separated_pair(
        separated_list1(newline, nom_register),
        many1(newline),
        nom_program,
    )(input)
    .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    let regs: HashMap<char, u64> = registers.iter().cloned().collect();

    let cpu = CPU::new(
        *regs.get(&'A').unwrap_or(&0),
        *regs.get(&'B').unwrap_or(&0),
        *regs.get(&'C').unwrap_or(&0),
        program,
    );

    Ok(cpu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_register() {
        assert_eq!(nom_register("Register A: 1"), Ok(("", ('A', 1))));
        assert_eq!(nom_register("Register B: 2"), Ok(("", ('B', 2))));
        assert_eq!(nom_register("Register C: 321"), Ok(("", ('C', 321))));
    }

    #[test]
    fn test_nom_program() {
        assert_eq!(nom_program("Program: 1,2,3"), Ok(("", vec![1, 2, 3])));
    }

    #[test]
    fn test_parse_input() {
        let input = ["Register A: 2", "Register B: 15", "", "Program: 1,2,3"].join("\n");

        let cpu = parse_input(&input).unwrap();

        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.b, 15);
        assert_eq!(cpu.c, 0); // Default value
        assert_eq!(cpu.program, vec![1, 2, 3]);
    }
}
