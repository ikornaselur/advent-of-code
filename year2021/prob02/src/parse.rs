use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_direction(input: &str) -> IResult<&str, GridDirection> {
    alt((
        value(GridDirection::Up, tag("up")),
        value(GridDirection::Down, tag("down")),
        value(GridDirection::Right, tag("forward")),
    ))
    .parse(input)
}

fn nom_instruction(input: &str) -> IResult<&str, (GridDirection, usize)> {
    separated_pair(nom_direction, tag(" "), nom_unsigned_digit).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<(GridDirection, usize)>> {
    let (_, instructions) = separated_list1(newline, nom_instruction)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_direction() {
        assert_eq!(nom_direction("up"), Ok(("", GridDirection::Up)));
        assert_eq!(nom_direction("down"), Ok(("", GridDirection::Down)));
        assert_eq!(nom_direction("forward"), Ok(("", GridDirection::Right)));
    }

    #[test]
    fn test_nom_instruction() {
        assert_eq!(
            nom_instruction("forward 5"),
            Ok(("", (GridDirection::Right, 5)))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "forward 5\ndown 3";
        assert_eq!(
            parse_input(input).unwrap(),
            vec![(GridDirection::Right, 5), (GridDirection::Down, 3)]
        );
    }
}
