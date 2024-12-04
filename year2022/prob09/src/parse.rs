use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_instruction(input: &str) -> IResult<&str, (GridDirection, usize)> {
    let (input, (direction, steps)) =
        separated_pair(alpha1, space1, nom_unsigned_digit::<usize>)(input)?;

    let direction = GridDirection::from_udlr(direction).unwrap();

    Ok((input, (direction, steps)))
}

pub fn parse_instructions(input: &str) -> Result<Vec<(GridDirection, usize)>> {
    let mut parser = separated_list1(newline, nom_instruction);

    let (_, instructions) = parser(input).map_err(|e| error!("Unable to parse: {}", e))?;

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_instruction() {
        assert_eq!(nom_instruction("U 123"), Ok(("", (GridDirection::Up, 123))));
    }

    #[test]
    fn test_parse_instructions() {
        let input = "U 1\nD 3\nL 1";
        let instructions = parse_instructions(input).unwrap();

        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0], (GridDirection::Up, 1));
        assert_eq!(instructions[1], (GridDirection::Down, 3));
        assert_eq!(instructions[2], (GridDirection::Left, 1));
    }
}
