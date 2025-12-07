use crate::cpu::Instruction;
use advent::parsers::nom_signed_digit;
use advent::prelude::*;

fn nom_instruction(input: &str) -> IResult<&str, Instruction> {
    let addx_parser = map(
        preceded(tag("addx "), nom_signed_digit::<i32>),
        Instruction::AddX,
    );
    let noop_parser = value(Instruction::Noop, tag("noop"));

    alt((addx_parser, noop_parser)).parse(input)
}

pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>> {
    let mut parser = separated_list1(newline, nom_instruction);

    let (_, instructions) = parser
        .parse(input)
        .map_err(|e| error!("Unable to parse: {}", e))?;

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_instruction_addx() {
        let (rest, result) = nom_instruction("addx 42").unwrap();

        assert_eq!(rest, "");
        assert_eq!(result, Instruction::AddX(42));

        let (rest, result) = nom_instruction("addx -3").unwrap();

        assert_eq!(rest, "");
        assert_eq!(result, Instruction::AddX(-3));
    }

    #[test]
    fn test_nom_instruction_noop() {
        let (rest, result) = nom_instruction("noop").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, Instruction::Noop);
    }

    #[test]
    fn test_parse_instructions() {
        let input = "addx 3\nnoop\nnoop\naddx -1";
        let result = parse_instructions(input).unwrap();

        assert_eq!(
            result,
            vec![
                Instruction::AddX(3),
                Instruction::Noop,
                Instruction::Noop,
                Instruction::AddX(-1),
            ]
        );
    }
}
