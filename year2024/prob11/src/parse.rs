use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

pub fn parse_input(input: &str) -> Result<Vec<usize>> {
    let (_, digits) = separated_list1(space1, nom_unsigned_digit)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("1 2 415").unwrap(), vec![1, 2, 415]);
    }
}
