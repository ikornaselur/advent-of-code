use advent::parsers::nom_signed_digit;
use advent::prelude::*;

fn nom_comma_sep(input: &str) -> IResult<&str, Coordinate<i32>> {
    separated_pair(nom_signed_digit, tag(","), nom_signed_digit)(input)
}

fn nom_state(input: &str) -> IResult<&str, (Coordinate<i32>, Coordinate<i32>)> {
    let (input, pos) = preceded(tag("p="), nom_comma_sep)(input)?;
    let (input, vector) = preceded(tag(" v="), nom_comma_sep)(input)?;
    Ok((input, (pos, vector)))
}

pub fn parse_input(input: &str) -> Result<Vec<(Coordinate<i32>, Coordinate<i32>)>> {
    let (_, states) = separated_list1(newline, nom_state)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(states)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_comma_sep() {
        assert_eq!(nom_comma_sep("1,2"), Ok(("", (1, 2))));
    }

    #[test]
    fn test_nom_state() {
        assert_eq!(nom_state("p=1,2 v=3,4"), Ok(("", ((1, 2), (3, 4)))));
    }

    #[test]
    fn test_parse_input() {
        let input = ["p=1,2 v=3,-2", "p=4,5 v=6,7"].join("\n");
        let output = parse_input(&input).unwrap();

        assert_eq!(output, vec![((1, 2), (3, -2)), ((4, 5), (6, 7))]);
    }
}
