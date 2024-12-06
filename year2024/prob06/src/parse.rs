use crate::{Grid, Node};
use advent::prelude::*;

fn nom_node(input: &str) -> IResult<&str, Node> {
    alt((
        value(Node::Open, char('.')),
        value(Node::Obsticle, char('#')),
        // We know the guard starts facin up
        value(Node::Guard, char('^')),
    ))(input)
}

fn nom_row(input: &str) -> IResult<&str, Vec<Node>> {
    many1(nom_node)(input)
}

fn nom_grid(input: &str) -> IResult<&str, Vec<Vec<Node>>> {
    separated_list1(newline, nom_row)(input)
}

pub fn parse_input(input: &str) -> Result<Grid> {
    let (_, nodes) = nom_grid(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    let guard = nodes
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, node)| {
                if let Node::Guard = node {
                    // As usual, we always use (row, column).. maybe I should start using that
                    // instead of the letters x and y
                    Some((y as i32, x as i32))
                } else {
                    None
                }
            })
        })
        .ok_or(error!("No guard found in input"))?;

    let height = nodes.len() as i32;
    let width = nodes[0].len() as i32;

    Ok(Grid {
        nodes,
        height,
        width,
        guard,
        guard_direction: GridDirection::Up,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_node() {
        assert_eq!(nom_node("."), Ok(("", Node::Open)));
        assert_eq!(nom_node("#"), Ok(("", Node::Obsticle)));
        assert_eq!(nom_node("^"), Ok(("", Node::Guard)));
    }

    #[test]
    fn test_nom_row() {
        assert_eq!(
            nom_row(".#..^."),
            Ok((
                "",
                vec![
                    Node::Open,
                    Node::Obsticle,
                    Node::Open,
                    Node::Open,
                    Node::Guard,
                    Node::Open,
                ]
            ))
        );
    }

    #[test]
    fn test_nom_grid() {
        assert_eq!(
            nom_grid(".#.\n#^."),
            Ok((
                "",
                vec![
                    vec![Node::Open, Node::Obsticle, Node::Open],
                    vec![Node::Obsticle, Node::Guard, Node::Open],
                ]
            ))
        )
    }

    #[test]
    fn test_parse_input() {
        let input = ".#.\n#.^";
        let grid = parse_input(input).unwrap();

        assert_eq!(
            grid.nodes,
            vec![
                vec![Node::Open, Node::Obsticle, Node::Open],
                vec![Node::Obsticle, Node::Open, Node::Guard],
            ]
        );
        assert_eq!(grid.guard, (1, 2));
        assert_eq!(grid.guard_direction, GridDirection::Up);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
    }
}
