use advent::prelude::*;

fn nom_bits(input: &str) -> IResult<&str, Vec<bool>> {
    many1(alt((value(false, char('0')), value(true, char('1'))))).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<bool>>> {
    let (_, bit_list) = separated_list1(newline, nom_bits)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(bit_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_row() {
        assert_eq!(nom_bits("000"), Ok(("", vec![false, false, false])));
        assert_eq!(nom_bits("110"), Ok(("", vec![true, true, false])));
    }

    #[test]
    fn test_parse_input() {
        let input = "001\n101";
        assert_eq!(
            parse_input(input).unwrap(),
            vec![vec![false, false, true], vec![true, false, true]]
        );
    }
}
