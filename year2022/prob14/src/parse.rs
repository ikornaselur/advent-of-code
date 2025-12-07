use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_coordinate(input: &str) -> IResult<&str, GridCoordinate<usize>> {
    map(
        separated_pair(
            nom_unsigned_digit::<usize>,
            delimited(multispace0, char(','), multispace0),
            nom_unsigned_digit::<usize>,
        ),
        |(column, row)| GridCoordinate { row, column },
    )
    .parse(input)
}

fn nom_coordinate_list(input: &str) -> IResult<&str, Vec<GridCoordinate<usize>>> {
    separated_list1(tag(" -> "), nom_coordinate).parse(input)
}

pub fn parse_coordinate_lists(input: &str) -> Result<Vec<Vec<GridCoordinate<usize>>>> {
    let (_, coordinates) = separated_list1(newline, nom_coordinate_list)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse heightmap: {:?}", e)))?;

    Ok(coordinates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_coordinate() {
        assert_eq!(
            nom_coordinate("123,456"),
            Ok((
                "",
                GridCoordinate {
                    column: 123,
                    row: 456
                }
            ))
        );
        assert_eq!(
            nom_coordinate("123, 456"),
            Ok((
                "",
                GridCoordinate {
                    column: 123,
                    row: 456
                }
            ))
        );
        assert_eq!(
            nom_coordinate("123,456 "),
            Ok((
                " ",
                GridCoordinate {
                    column: 123,
                    row: 456
                }
            ))
        );
    }

    #[test]
    fn test_nom_coordinate_list() {
        assert_eq!(
            nom_coordinate_list("1,2 -> 3,4 -> 5,6"),
            Ok((
                "",
                vec![
                    GridCoordinate { column: 1, row: 2 },
                    GridCoordinate { column: 3, row: 4 },
                    GridCoordinate { column: 5, row: 6 }
                ]
            ))
        );
    }

    #[test]
    fn test_parse_coordinates_lists() {
        let input = ["1,2 -> 3,4 -> 5,6", "7,8 -> 9,10"].join("\n");
        let result = parse_coordinate_lists(&input).unwrap();

        assert_eq!(
            result,
            vec![
                vec![
                    GridCoordinate { column: 1, row: 2 },
                    GridCoordinate { column: 3, row: 4 },
                    GridCoordinate { column: 5, row: 6 },
                ],
                vec![
                    GridCoordinate { column: 7, row: 8 },
                    GridCoordinate { column: 9, row: 10 },
                ],
            ]
        );
    }
}
