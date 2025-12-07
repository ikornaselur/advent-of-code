use advent::prelude::*;

fn nom_char(input: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ").parse(input)
}

fn nom_char_row(input: &str) -> IResult<&str, Vec<char>> {
    many1(nom_char).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    let (_, plot_map) = separated_list1(newline, nom_char_row)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(plot_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_char() {
        assert_eq!(nom_char("AB"), Ok(("B", 'A')));
    }

    #[test]
    fn test_nom_char_row() {
        assert_eq!(nom_char_row("AB"), Ok(("", vec!['A', 'B'])));
    }

    #[test]
    fn test_parse_input() {
        let input = "AB\nCD";
        let output = parse_input(input).unwrap();
        assert_eq!(output, vec![vec!['A', 'B'], vec!['C', 'D']]);
    }
}
