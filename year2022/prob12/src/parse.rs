use crate::{Heightmap, Point};
use advent::prelude::*;

fn nom_point(input: &str) -> IResult<&str, Point> {
    let char_parser = map(one_of("abcdefghijklmnopqrstuvwxyz"), |c| {
        Point::Height(c as i32 - 'a' as i32)
    });

    alt((
        char_parser,
        value(Point::Start, char('S')),
        value(Point::End, char('E')),
    ))(input)
}

/// Parse a heightmap row, which is made out of characters from a to z (lowercase).
///
/// 'a' is considered the lowest at 0 and 'z' the highest at 25.
/// Note that there are two special characters, upper case 'S' for start and upper case 'E' for
/// end.
fn nom_heighmap_row(input: &str) -> IResult<&str, Vec<Point>> {
    many1(nom_point)(input)
}

/// Parse a full heighmap from a string into a matrix
pub fn parse_heightmap(input: &str) -> Result<Heightmap> {
    let (_, points) = separated_list1(newline, nom_heighmap_row)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse heightmap: {:?}", e)))?;

    Ok(Heightmap::new(points))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_char() {
        assert_eq!(nom_point("a"), Ok(("", Point::Height(0))));
        assert_eq!(nom_point("z"), Ok(("", Point::Height(25))));
        assert_eq!(nom_point("S"), Ok(("", Point::Start)));
        assert_eq!(nom_point("E"), Ok(("", Point::End)));
    }

    #[test]
    fn test_nom_heighmap_row() {
        assert_eq!(
            nom_heighmap_row("aSbE"),
            Ok((
                "",
                vec![Point::Height(0), Point::Start, Point::Height(1), Point::End]
            ))
        );
    }

    #[test]
    fn test_parse_heightmap() {
        let input = ["aSbe", "cdEf"].join("\n");
        let heightmap = parse_heightmap(&input).unwrap();

        assert_eq!(
            heightmap.map,
            vec![
                vec![
                    Point::Height(0),
                    Point::Start,
                    Point::Height(1),
                    Point::Height(4)
                ],
                vec![
                    Point::Height(2),
                    Point::Height(3),
                    Point::End,
                    Point::Height(5)
                ],
            ]
        );
        assert_eq!(heightmap.end, GridCoordinate { row: 1, column: 2 });
    }
}
