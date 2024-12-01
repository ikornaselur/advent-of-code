use advent::prelude::*;

fn nom_line(input: &str) -> IResult<&str, (i32, i32)> {
    map_res(
        separated_pair(digit1, multispace1, digit1),
        |(a, b): (&str, &str)| -> Result<(i32, i32)> { Ok((a.parse::<i32>()?, b.parse::<i32>()?)) },
    )(input)
}

pub fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    let (_, pairs) = separated_list1(newline, nom_line)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

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
