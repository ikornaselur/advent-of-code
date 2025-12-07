use advent::prelude::*;

fn nom_line(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<usize>> {
    let (_, pairs) = separated_list1(newline, nom_line)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "199\n200\n201";
        assert_eq!(parse_input(input).unwrap(), vec![199, 200, 201]);
    }
}
