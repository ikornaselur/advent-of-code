use nom::{
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult, Parser,
};
use num_traits::{Signed, Unsigned};
use std::str::FromStr;

pub fn nom_unsigned_digit<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr + Unsigned,
{
    map_res(digit1, |s: &str| s.parse::<T>()).parse(input)
}

pub fn nom_signed_digit<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr + Signed,
{
    map_res(recognize(preceded(opt(char('-')), digit1)), |s: &str| {
        s.parse::<T>()
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_unsigned_digit() {
        assert_eq!(nom_unsigned_digit::<u64>("123"), Ok(("", 123u64)));
        assert_eq!(nom_unsigned_digit::<u64>("123abc"), Ok(("abc", 123u64)));
        assert_eq!(nom_unsigned_digit::<usize>("123"), Ok(("", 123usize)));
        assert!(nom_unsigned_digit::<u32>("-123").is_err());
    }

    #[test]
    fn test_nom_signed_digit() {
        assert_eq!(nom_signed_digit::<i64>("123"), Ok(("", 123i64)));
        assert_eq!(nom_signed_digit::<i64>("-123"), Ok(("", -123i64)));
    }
}
