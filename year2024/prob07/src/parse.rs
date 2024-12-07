use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_line(input: &str) -> IResult<&str, (usize, Vec<usize>)> {
    separated_pair(
        nom_unsigned_digit,
        tag(": "),
        separated_list1(space1, nom_unsigned_digit),
    )(input)
}

pub fn parse_input(input: &str) -> Result<Vec<(usize, Vec<usize>)>> {
    let (_, inputs) = separated_list1(newline, nom_line)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_line() {
        assert_eq!(nom_line("123: 2 3 4"), Ok(("", (123, vec![2, 3, 4]))));
    }

    #[test]
    fn test_parse_input() {
        let input = "1: 2 3\n45: 2 31";
        let output = parse_input(input).unwrap();
        assert_eq!(output, vec![(1, vec![2, 3]), (45, vec![2, 31])]);
    }
}
