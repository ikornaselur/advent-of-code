use advent::parsers::nom_unsigned_digit;
use std::ops::RangeInclusive;

use advent::prelude::*;

pub type FreshRange = RangeInclusive<u64>;

fn nom_range(input: &str) -> IResult<&str, FreshRange> {
    let (input, start) = nom_unsigned_digit(input)?;
    let (input, _) = char('-')(input)?;
    let (input, end) = nom_unsigned_digit(input)?;

    Ok((input, start..=end))
}

fn nom_ranges_list(input: &str) -> IResult<&str, Vec<FreshRange>> {
    separated_list1(newline, nom_range).parse(input)
}

fn nom_ingredient_ids(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(newline, nom_unsigned_digit).parse(input)
}

pub fn parse_input(input: &str) -> Result<(Vec<FreshRange>, Vec<u64>)> {
    let (_, (fresh_ranges, ingredient_ids)) =
        separated_pair(nom_ranges_list, many1(newline), nom_ingredient_ids).parse(input)?;

    Ok((fresh_ranges, ingredient_ids))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_range() {
        assert_eq!(nom_range("4-8"), Ok(("", 4..=8)));
    }

    #[test]
    fn test_nom_ranges_list() {
        let input = "1-2\n3-4\n5-6\n\n123";
        let (rest, ranges) = nom_ranges_list(input).unwrap();

        assert_eq!(rest, "\n\n123");
        assert_eq!(ranges, vec![1..=2, 3..=4, 5..=6,]);
    }

    #[test]
    fn test_nom_ingredient_ids() {
        assert_eq!(nom_ingredient_ids("1\n2\n3"), Ok(("", vec![1, 2, 3])))
    }

    #[test]
    fn test_parse_input() {
        let input = "1-2\n3-4\n\n1\n2\n3";
        let (fresh_ranges, ingredient_ids) = parse_input(input).unwrap();

        assert_eq!(fresh_ranges, vec![1..=2, 3..=4]);
        assert_eq!(ingredient_ids, vec![1, 2, 3]);
    }
}
