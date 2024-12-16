use crate::{Map, Node};
use advent::prelude::*;

fn nom_node(input: &str) -> IResult<&str, Node> {
    alt((
        value(Node::Wall, char('#')),
        value(Node::Floor, char('.')),
        value(Node::Start, char('S')),
        value(Node::End, char('E')),
    ))(input)
}

fn nom_row(input: &str) -> IResult<&str, Vec<Node>> {
    many1(nom_node)(input)
}

fn nom_map(input: &str) -> IResult<&str, Vec<Vec<Node>>> {
    separated_list1(newline, nom_row)(input)
}

pub fn parse_input(input: &str) -> Result<Map> {
    let (_, nodes) = nom_map(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(Map::new(nodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_node() {
        assert_eq!(nom_node("#.S"), Ok((".S", Node::Wall)));
        assert_eq!(nom_node(".#S"), Ok(("#S", Node::Floor)));
        assert_eq!(nom_node("S.#"), Ok((".#", Node::Start)));
    }

    #[test]
    fn test_nom_row() {
        assert_eq!(
            nom_row("#.S"),
            Ok(("", vec![Node::Wall, Node::Floor, Node::Start]))
        );
    }

    #[test]
    fn test_nom_map() {
        assert_eq!(
            nom_map("#.S\n.#E"),
            Ok((
                "",
                vec![
                    vec![Node::Wall, Node::Floor, Node::Start],
                    vec![Node::Floor, Node::Wall, Node::End]
                ]
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("#.S\n.#E").unwrap(),
            Map::new(vec![
                vec![Node::Wall, Node::Floor, Node::Start],
                vec![Node::Floor, Node::Wall, Node::End]
            ])
        );
    }
}
