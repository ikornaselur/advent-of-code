use advent::prelude::*;

fn nom_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn nom_valid_mul(input: &str) -> IResult<&str, (usize, usize)> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(nom_number, tag(","), nom_number),
            tag(")"),
        ),
    )(input)
}

fn nom_muls(input: &str) -> IResult<&str, (usize, usize)> {
    alt((nom_valid_mul, preceded(take(1usize), nom_muls)))(input)
}

pub fn parse_input(input: &str) -> Result<Vec<(usize, usize)>> {
    let mut parser = many1(nom_muls);
    let (_, digit_pairs) = parser(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(digit_pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_number() {
        assert_eq!(nom_number("1243"), Ok(("", 1243)));
    }

    #[test]
    fn test_nom_valid_mul() {
        assert_eq!(nom_valid_mul("mul(123,456)"), Ok(("", (123, 456))));
        assert!(nom_valid_mul("mul(123, 456)").is_err());
    }

    #[test]
    fn test_nom_muls() {
        assert_eq!(nom_muls("mul(1,2)"), Ok(("", (1, 2))));
        assert_eq!(nom_muls("mul(1,2)mul(3,4)"), Ok(("mul(3,4)", (1, 2))));
        assert_eq!(nom_muls("!@$mul(1,2)ASF"), Ok(("ASF", (1, 2))));
    }

    #[test]
    fn test_parse_input() {
        let input = "xmul(123,456)%!@do_not_mul(3,4)+mul(32,54]then(mul(1,2)";
        assert_eq!(
            parse_input(input).unwrap(),
            vec![(123, 456), (3, 4), (1, 2)]
        );
    }
}
