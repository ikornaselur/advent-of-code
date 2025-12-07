use advent::prelude::*;

fn nom_xmas(input: &str) -> IResult<&str, char> {
    alt((char('X'), char('M'), char('A'), char('S'))).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    let (_, grid) = separated_list1(newline, many1(nom_xmas))
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_xmas() {
        assert_eq!(nom_xmas("X"), Ok(("", 'X')));
        assert_eq!(nom_xmas("M"), Ok(("", 'M')));
        assert_eq!(nom_xmas("A"), Ok(("", 'A')));
        assert_eq!(nom_xmas("S"), Ok(("", 'S')));
    }

    #[test]
    fn test_parse_input() {
        let input = "XMAS\nSMAX";

        let grid = parse_input(input).unwrap();

        assert_eq!(
            grid,
            vec![vec!['X', 'M', 'A', 'S'], vec!['S', 'M', 'A', 'X']]
        );
    }
}
