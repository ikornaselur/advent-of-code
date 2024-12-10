use advent::prelude::*;

fn nom_row(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of("0123456789"))(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    let (_, map) = separated_list1(newline, nom_row)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "0123\n5123";
        let result = parse_input(input).unwrap();

        assert_eq!(
            result,
            vec![vec!['0', '1', '2', '3'], vec!['5', '1', '2', '3']]
        );
    }
}
