use crate::{Cell, Grid};
use advent::prelude::*;

fn nom_empty(input: &str) -> IResult<&str, Cell> {
    value(Cell::Empty, char('.')).parse(input)
}
fn nom_paper(input: &str) -> IResult<&str, Cell> {
    value(Cell::Paper, char('@')).parse(input)
}

fn nom_cell(input: &str) -> IResult<&str, Cell> {
    alt((nom_empty, nom_paper)).parse(input)
}

fn nom_row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(nom_cell).parse(input)
}

pub fn parse_input(input: &str) -> Result<Grid> {
    let (_, grid) = separated_list1(newline, nom_row).parse(input)?;

    Ok(Grid::new(grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_cell() {
        assert_eq!(nom_cell("."), Ok(("", Cell::Empty)));
        assert_eq!(nom_cell("@"), Ok(("", Cell::Paper)));
    }

    #[test]
    fn test_parse_input() {
        let input = "..@\n@.@";
        let grid = parse_input(input).unwrap();
        assert_eq!(
            grid,
            Grid::new(vec![
                vec![Cell::Empty, Cell::Empty, Cell::Paper],
                vec![Cell::Paper, Cell::Empty, Cell::Paper]
            ])
        );
    }
}
