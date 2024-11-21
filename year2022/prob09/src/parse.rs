use advent::prelude::*;
use nom::{
    character::complete::{alpha1, digit1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn nom_instruction(input: &str) -> IResult<&str, (OrdinalDirection, usize)> {
    let (input, (direction, steps)) = separated_pair(alpha1, space1, digit1)(input)?;

    let direction = OrdinalDirection::from_udlr(direction).unwrap();

    Ok((input, (direction, steps.parse().unwrap())))
}

pub fn parse_instructions(input: &str) -> Result<Vec<(OrdinalDirection, usize)>> {
    let mut parser = separated_list1(newline, nom_instruction);

    let (_, instructions) = parser(input).map_err(|e| error!("Unable to parse: {}", e))?;

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_instruction() {
        assert_eq!(
            nom_instruction("U 123"),
            Ok(("", (OrdinalDirection::Up, 123)))
        );
    }

    #[test]
    fn test_parse_instructions() {
        let input = "U 1\nD 3\nL 1";
        let instructions = parse_instructions(input).unwrap();

        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0], (OrdinalDirection::Up, 1));
        assert_eq!(instructions[1], (OrdinalDirection::Down, 3));
        assert_eq!(instructions[2], (OrdinalDirection::Left, 1));
    }
}
