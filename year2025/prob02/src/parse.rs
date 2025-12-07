use advent::{parsers::nom_unsigned_digit, prelude::*};
use std::ops::RangeInclusive;

pub type IDRange = RangeInclusive<u64>;

fn nom_range(input: &str) -> IResult<&str, IDRange> {
    let (input, start) = nom_unsigned_digit(input)?;
    let (input, _) = char('-')(input)?;
    let (input, end) = nom_unsigned_digit(input)?;

    Ok((input, start..=end))
}

fn nom_ranges(input: &str) -> IResult<&str, Vec<IDRange>> {
    separated_list1(char(','), nom_range).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<IDRange>> {
    let (_, ranges) = nom_ranges(input)?;

    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_range() {
        let input = "11-22";
        let (_, range) = nom_range(input).unwrap();
        assert_eq!(range, 11..=22);
    }

    #[test]
    fn test_nom_ranges() {
        let input = "11-22,998-1012";
        let (_, ranges) = nom_ranges(input).unwrap();
        assert_eq!(ranges, vec![11..=22, 998..=1012]);
    }

    #[test]
    fn test_parse_input() {
        let input = "11-22,998-1012";
        let (_, ranges) = nom_ranges(input).unwrap();
        assert_eq!(ranges, vec![11..=22, 998..=1012]);
    }
}
