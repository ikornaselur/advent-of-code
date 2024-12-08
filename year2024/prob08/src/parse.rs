use crate::Node;
use advent::prelude::*;

fn nom_node(input: &str) -> IResult<&str, Node> {
    alt((
        value(Node::Empty, char('.')),
        map(satisfy(|c: char| c.is_alphanumeric()), Node::Antenna),
    ))(input)
}

fn nom_row(input: &str) -> IResult<&str, Vec<Node>> {
    many1(nom_node)(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<Node>>> {
    let mut parser = separated_list1(newline, nom_row);
    let (_, antenna_map) = parser(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(antenna_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_node() {
        assert_eq!(nom_node(".x"), Ok(("x", Node::Empty)));
        assert_eq!(nom_node("x"), Ok(("", Node::Antenna('x'))));
        assert!(nom_node("\n").is_err());
    }

    #[test]
    fn test_nom_row() {
        assert_eq!(
            nom_row(".x\ny."),
            Ok(("\ny.", vec![Node::Empty, Node::Antenna('x')]))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = ".x\ny.";
        let output = parse_input(input).unwrap();

        assert_eq!(
            output,
            vec![
                vec![Node::Empty, Node::Antenna('x')],
                vec![Node::Antenna('y'), Node::Empty]
            ]
        );
    }
}
