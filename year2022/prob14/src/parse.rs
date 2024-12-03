use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_coordinate(input: &str) -> IResult<&str, Coordinate<usize>> {
    separated_pair(
        nom_unsigned_digit::<usize>,
        delimited(multispace0, char(','), multispace0),
        nom_unsigned_digit::<usize>,
    )(input)
}

fn nom_coordinate_list(input: &str) -> IResult<&str, Vec<Coordinate<usize>>> {
    separated_list1(tag(" -> "), nom_coordinate)(input)
}

pub fn parse_coordinate_lists(input: &str) -> Result<Vec<Vec<Coordinate<usize>>>> {
    let (_, coordinates) = separated_list1(newline, nom_coordinate_list)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse heightmap: {:?}", e)))?;

    Ok(coordinates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_coordinate() {
        assert_eq!(nom_coordinate("123,456"), Ok(("", (123, 456))));
        assert_eq!(nom_coordinate("123, 456"), Ok(("", (123, 456))));
        assert_eq!(nom_coordinate("123,456 "), Ok((" ", (123, 456))));
    }

    #[test]
    fn test_nom_coordinate_list() {
        assert_eq!(
            nom_coordinate_list("1,2 -> 3,4 -> 5,6"),
            Ok(("", vec![(1, 2), (3, 4), (5, 6)]))
        );
    }

    #[test]
    fn test_parse_coordinates_lists() {
        let input = ["1,2 -> 3,4 -> 5,6", "7,8 -> 9,10"].join("\n");
        let result = parse_coordinate_lists(&input).unwrap();

        assert_eq!(
            result,
            vec![vec![(1, 2), (3, 4), (5, 6)], vec![(7, 8), (9, 10)],]
        );
    }
}
