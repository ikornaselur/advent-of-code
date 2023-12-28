use crate::Category;
use advent::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, one_of},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::cmp::Ordering;

/// XMAS Part
///
/// A part is an object with four keys, x, m, a and s, where each value is an integer, like this:
///
/// {x=787,m=2655,a=1222,s=2876}
pub fn xmas(input: &str) -> IResult<&str, (u32, u32, u32, u32)> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, (x, m, a, s)))
}

fn category_from_char(c: char) -> Result<Category> {
    match c {
        'x' => Ok(Category::X),
        'm' => Ok(Category::M),
        'a' => Ok(Category::A),
        's' => Ok(Category::S),
        _ => Err(error!("Invalid category")),
    }
}

type ConditionTuple = (Category, Ordering, u32);
type ConditionsVec<'a> = Vec<(Option<ConditionTuple>, &'a str)>;

/// Parse condition
///
/// A condition is a letter, followed by a comparison operator, followed by a number, like this:
///
///     * a>1716
///     * b<519
fn parse_condition_part(input: &str) -> IResult<&str, (Category, Ordering, u32)> {
    let parse_ordering = alt((
        map(tag(">"), |_| Ordering::Greater),
        map(tag("<"), |_| Ordering::Less),
    ));

    let (input, (cond_char, ordering, number)) = tuple((
        map_res(one_of("xmas"), category_from_char),
        parse_ordering,
        map_res(digit1, str::parse),
    ))(input)?;

    Ok((input, (cond_char, ordering, number)))
}

pub fn condition(input: &str) -> IResult<&str, (Option<ConditionTuple>, &str)> {
    let (input, condition) = opt(terminated(parse_condition_part, char(':')))(input)?;
    let (input, final_string) = alpha1(input)?;

    Ok((input, (condition, final_string)))
}

pub fn conditions(input: &str) -> IResult<&str, ConditionsVec> {
    separated_list1(char(','), condition)(input)
}

pub fn workflow(input: &str) -> IResult<&str, (&str, ConditionsVec)> {
    tuple((alpha1, delimited(char('{'), conditions, char('}'))))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas() {
        assert_eq!(
            xmas("{x=787,m=2655,a=1222,s=2876}"),
            Ok(("", (787u32, 2655u32, 1222u32, 2876u32)))
        );
    }

    #[test]
    fn test_parse_condition_part() {
        assert_eq!(
            parse_condition_part("x>1716:A"),
            Ok((":A", (Category::X, Ordering::Greater, 1716)))
        );
    }

    #[test]
    fn test_condition() {
        assert_eq!(
            condition("m>1716:A"),
            Ok(("", (Some((Category::M, Ordering::Greater, 1716)), "A")))
        )
    }

    #[test]
    fn test_conditions() {
        assert_eq!(
            conditions("m>1716:A,x<1716:B,R"),
            Ok((
                "",
                vec![
                    (Some((Category::M, Ordering::Greater, 1716)), "A"),
                    (Some((Category::X, Ordering::Less, 1716)), "B"),
                    (None, "R")
                ]
            ))
        )
    }

    #[test]
    fn test_workflow() {
        assert_eq!(
            workflow("A{m>1716:A,x<1716:B,R}"),
            Ok((
                "",
                (
                    "A",
                    vec![
                        (Some((Category::M, Ordering::Greater, 1716)), "A"),
                        (Some((Category::X, Ordering::Less, 1716)), "B"),
                        (None, "R")
                    ]
                )
            ))
        )
    }
}
