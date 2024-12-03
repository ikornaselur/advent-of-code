use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

use crate::Range;

fn nom_range(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(
        nom_unsigned_digit::<u32>,
        char('-'),
        nom_unsigned_digit::<u32>,
    )(input)?;

    Ok((input, Range { start, end }))
}

fn nom_range_pair(input: &str) -> IResult<&str, (Range, Range)> {
    let (input, (range1, range2)) = separated_pair(nom_range, char(','), nom_range)(input)?;
    Ok((input, (range1, range2)))
}

pub fn parse_range_pair(input: &str) -> Result<(Range, Range)> {
    let (_, (range1, range2)) =
        nom_range_pair(input).map_err(|e| error!("Unable to parse: {}", e))?;

    Ok((range1, range2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        assert_eq!(nom_range("1-3"), Ok(("", Range { start: 1, end: 3 })));
    }

    #[test]
    fn test_parse_range_pair() {
        assert_eq!(
            nom_range_pair("1-3,4-6"),
            Ok(("", (Range { start: 1, end: 3 }, Range { start: 4, end: 6 })))
        );
    }
}
