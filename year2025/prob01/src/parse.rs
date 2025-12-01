use advent::parsers::nom_signed_digit;
use advent::prelude::*;

fn nom_line(input: &str) -> IResult<&str, i32> {
    let (input, dir) = nom_direction(input)?;
    let (input, digit) = nom_signed_digit::<i32>(input)?;

    Ok((input, digit * dir))
}

fn nom_direction(input: &str) -> IResult<&str, i32> {
    alt((value(1, tag("R")), value(-1, tag("L"))))(input)
}

pub fn parse_input(input: &str) -> Result<Vec<i32>> {
    let (_, instructions) = separated_list1(newline, nom_line)(input)?;

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_direction() {
        assert_eq!(nom_direction("L"), Ok(("", -1)));
        assert_eq!(nom_direction("R"), Ok(("", 1)));
    }

    #[test]
    fn test_nom_line() {
        assert_eq!(nom_line("L12"), Ok(("", -12)));
        assert_eq!(nom_line("R26"), Ok(("", 26)));
    }

    #[test]
    fn test_parse_input() {
        let input = "L13\nR10";
        assert_eq!(parse_input(input).unwrap(), vec![-13, 10]);
    }
}
