use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

pub fn parse_input(input: &str) -> Result<Vec<usize>> {
    let (_, digits) = separated_list1(newline, nom_unsigned_digit)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(digits)
}
