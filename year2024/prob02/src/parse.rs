use advent::parsers::nom_signed_digit;
use advent::prelude::*;

fn nom_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, nom_signed_digit::<i32>)(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    let (_, pairs) = separated_list1(newline, nom_line)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_line() {
        assert_eq!(nom_line("1 2").unwrap(), ("", vec![1, 2]));
        assert_eq!(nom_line("1   2").unwrap(), ("", vec![1, 2]));
    }

    #[test]
    fn test_parse_input() {
        let input = "1 2\n3 4";
        assert_eq!(parse_input(input).unwrap(), vec![vec![1, 2], vec![3, 4]]);
    }
}
