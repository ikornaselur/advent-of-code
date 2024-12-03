use crate::Instruction;
use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_valid_mul(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("mul"),
            delimited(
                tag("("),
                separated_pair(
                    nom_unsigned_digit::<usize>,
                    tag(","),
                    nom_unsigned_digit::<usize>,
                ),
                tag(")"),
            ),
        ),
        |(a, b)| Instruction::Mul(a, b),
    )(input)
}

fn nom_do(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Do, tag("do()"))(input)
}

fn nom_dont(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Dont, tag("don't()"))(input)
}

fn nom_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        nom_valid_mul,
        nom_dont, // Check for don't before do, as do is a prefix of don't!
        nom_do,
        preceded(take(1usize), nom_instruction),
    ))(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    let mut parser = many1(nom_instruction);
    let (_, instructions) = parser(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_valid_mul() {
        assert_eq!(
            nom_valid_mul("mul(123,456)"),
            Ok(("", Instruction::Mul(123, 456)))
        );
        assert!(nom_valid_mul("mul(123, 456)").is_err());
    }

    #[test]
    fn test_nom_instruction() {
        assert_eq!(
            nom_instruction("mul(1,2)"),
            Ok(("", Instruction::Mul(1, 2)))
        );
        assert_eq!(
            nom_instruction("mul(1,2)mul(3,4)"),
            Ok(("mul(3,4)", Instruction::Mul(1, 2)))
        );
        assert_eq!(
            nom_instruction("!@$mul(1,2)ASF"),
            Ok(("ASF", Instruction::Mul(1, 2)))
        );

        assert_eq!(nom_instruction("do()"), Ok(("", Instruction::Do)));
        assert_eq!(nom_instruction("1@$%)(do()))"), Ok(("))", Instruction::Do)));
        assert_eq!(nom_instruction("don't()"), Ok(("", Instruction::Dont)));
        assert_eq!(nom_instruction("do(don't())"), Ok((")", Instruction::Dont)));
    }

    #[test]
    fn test_parse_input() {
        let input = "xmul(123,456)%!@do()do_not_mul(3,4)+don't()mul(32,54]then(mul(1,2)";
        assert_eq!(
            parse_input(input).unwrap(),
            vec![
                Instruction::Mul(123, 456),
                Instruction::Do,
                Instruction::Mul(3, 4),
                Instruction::Dont,
                Instruction::Mul(1, 2)
            ]
        );
    }
}
