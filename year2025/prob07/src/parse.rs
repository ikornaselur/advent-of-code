use crate::{Node, Nodes};
use advent::prelude::*;

fn nom_node(input: &str) -> IResult<&str, Node> {
    alt((
        value(Node::Start, char('S')),
        value(Node::Beam, char('|')),
        value(Node::Empty, char('.')),
        value(Node::Splitter, char('^')),
    ))
    .parse(input)
}

fn nom_nodes(input: &str) -> IResult<&str, Nodes> {
    many1(nom_node).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Nodes>> {
    let (_, grid) = separated_list1(newline, nom_nodes)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_node() {
        let input = ".|S^";

        let (input, node) = nom_node(input).unwrap();
        assert_eq!(input, "|S^");
        assert_eq!(node, Node::Empty);

        let (input, node) = nom_node(input).unwrap();
        assert_eq!(input, "S^");
        assert_eq!(node, Node::Beam);

        let (input, node) = nom_node(input).unwrap();
        assert_eq!(input, "^");
        assert_eq!(node, Node::Start);

        let (input, node) = nom_node(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(node, Node::Splitter);
    }

    #[test]
    fn test_nom_nodes() {
        let input = ".|S^";

        assert_eq!(
            nom_nodes(input),
            Ok((
                "",
                vec![Node::Empty, Node::Beam, Node::Start, Node::Splitter,]
            ))
        )
    }

    #[test]
    fn test_parse_input() {
        let input = ".S.\n.^.";

        let grid = parse_input(input).unwrap();

        assert_eq!(
            grid,
            vec![
                vec![Node::Empty, Node::Start, Node::Empty],
                vec![Node::Empty, Node::Splitter, Node::Empty],
            ]
        );
    }
}
