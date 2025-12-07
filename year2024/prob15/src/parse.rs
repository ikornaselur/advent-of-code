use crate::{Grid, Instructions, Node};
use advent::prelude::*;

fn nom_node(input: &str) -> IResult<&str, Node> {
    alt((
        value(Node::Wall, char('#')),
        value(Node::Box, char('O')),
        value(Node::Floor, char('.')),
        value(Node::Robot, char('@')),
    ))
    .parse(input)
}

fn nom_map(input: &str) -> IResult<&str, Vec<Vec<Node>>> {
    separated_list1(newline, many1(nom_node)).parse(input)
}

fn nom_instruction(input: &str) -> IResult<&str, GridDirection> {
    alt((
        value(GridDirection::Up, char('^')),
        value(GridDirection::Down, char('v')),
        value(GridDirection::Left, char('<')),
        value(GridDirection::Right, char('>')),
    ))
    .parse(input)
}

fn nom_instructions(input: &str) -> IResult<&str, Instructions> {
    many1(preceded(opt(newline), nom_instruction)).parse(input)
}

fn nom_input(input: &str) -> IResult<&str, Grid> {
    map(
        separated_pair(nom_map, newline, nom_instructions),
        |(nodes, instructions)| Grid::new(nodes, instructions),
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Result<Grid> {
    let (_, grid) = nom_input(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_node() {
        assert_eq!(nom_node("#").unwrap().1, Node::Wall);
        assert_eq!(nom_node("O").unwrap().1, Node::Box);
        assert_eq!(nom_node(".").unwrap().1, Node::Floor);
        assert_eq!(nom_node("@").unwrap().1, Node::Robot);
    }

    #[test]
    fn test_nom_map() {
        let input = ["####", "#.O#", "#@.#", "####"].join("\n");
        let map = vec![
            vec![Node::Wall, Node::Wall, Node::Wall, Node::Wall],
            vec![Node::Wall, Node::Floor, Node::Box, Node::Wall],
            vec![Node::Wall, Node::Robot, Node::Floor, Node::Wall],
            vec![Node::Wall, Node::Wall, Node::Wall, Node::Wall],
        ];
        assert_eq!(nom_map(&input).unwrap().1, map);
    }

    #[test]
    fn test_nom_instruction() {
        assert_eq!(nom_instruction("^").unwrap().1, GridDirection::Up);
        assert_eq!(nom_instruction("v").unwrap().1, GridDirection::Down);
        assert_eq!(nom_instruction("<").unwrap().1, GridDirection::Left);
        assert_eq!(nom_instruction(">").unwrap().1, GridDirection::Right);
    }

    #[test]
    fn test_nom_instructions() {
        let input = ["^v", "<>"].join("\n");
        let instructions = vec![
            GridDirection::Up,
            GridDirection::Down,
            GridDirection::Left,
            GridDirection::Right,
        ];
        assert_eq!(nom_instructions(&input).unwrap().1, instructions);
    }
}
