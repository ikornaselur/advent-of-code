use advent::parsers::nom_signed_digit;
use advent::prelude::*;

fn nom_coordinate(input: &str) -> IResult<&str, GridCoordinate<i32>> {
    // NOTE: The coordinates are (X,Y) where X is from left edge (column)
    // and Y is from top edge (row)
    map(
        separated_pair(nom_signed_digit, char(','), nom_signed_digit),
        |(column, row)| GridCoordinate { row, column },
    )(input)
}

pub fn parse_input(input: &str) -> Result<Vec<GridCoordinate<i32>>> {
    let (_, coordinates) = separated_list1(newline, nom_coordinate)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(coordinates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_coordinate() {
        assert_eq!(
            nom_coordinate("1,2"),
            Ok(("", GridCoordinate { row: 2, column: 1 }))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "1,2\n3,4\n";
        assert_eq!(
            parse_input(input).unwrap(),
            vec![
                GridCoordinate { row: 2, column: 1 },
                GridCoordinate { row: 4, column: 3 }
            ]
        );
    }
}
