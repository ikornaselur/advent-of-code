use crate::Computer;
use advent::prelude::*;

fn nom_computer(input: &str) -> IResult<&str, Computer> {
    map((anychar, anychar), |(a, b)| Computer { id: (a, b) }).parse(input)
}

fn nom_connection(input: &str) -> IResult<&str, (Computer, Computer)> {
    separated_pair(nom_computer, char('-'), nom_computer).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<(Computer, Computer)>> {
    let (_, connections) = separated_list1(newline, nom_connection)
        .parse(input.trim_end())
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(connections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_computer() {
        assert_eq!(
            nom_computer("ab-cd"),
            Ok(("-cd", Computer { id: ('a', 'b') }))
        );
    }

    #[test]
    fn test_nom_connection() {
        assert_eq!(
            nom_connection("ab-cd"),
            Ok((
                "",
                (Computer { id: ('a', 'b') }, Computer { id: ('c', 'd') })
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("ab-cd\nab-ef\n").unwrap(),
            vec![
                (Computer { id: ('a', 'b') }, Computer { id: ('c', 'd') }),
                (Computer { id: ('a', 'b') }, Computer { id: ('e', 'f') }),
            ]
        );
    }
}
