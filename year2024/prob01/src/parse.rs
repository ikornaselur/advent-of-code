use advent::parsers::nom_signed_digit;
use advent::prelude::*;

fn nom_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        nom_signed_digit::<i32>,
        multispace1,
        nom_signed_digit::<i32>,
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    let (_, pairs) = separated_list1(newline, nom_line).parse(input)?;

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_line() {
        assert_eq!(nom_line("1 2").unwrap(), ("", (1, 2)));
        assert_eq!(nom_line("1   2").unwrap(), ("", (1, 2)));
    }

    #[test]
    fn test_parse_input() {
        let input = "1 2\n3 4";
        assert_eq!(parse_input(input).unwrap(), vec![(1, 2), (3, 4)]);
    }
}
