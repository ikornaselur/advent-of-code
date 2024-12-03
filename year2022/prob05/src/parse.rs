use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

use crate::{Instruction, Stack};

/// Nom a single crate spot
///
/// Return none if empty, otherwise the character that was in the spot
fn nom_crate(input: &str) -> IResult<&str, Option<&str>> {
    let empty_parser = tag("   ");
    let crate_parser = delimited(char('['), take(1u8), char(']'));

    let (input, raw_crate) = alt((empty_parser, crate_parser))(input)?;

    match raw_crate {
        "   " => Ok((input, None)),
        c => Ok((input, Some(c))),
    }
}

/// Nom a line that looks like this:
///
///     [A] [B]     [C] [D]
///
/// which should return (assuming A is start of the line):
///
///     Some('A'), Some('B'), None, Some('C'), Some('D')
///
/// These are then used to create a list of stacks
fn nom_crate_row(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let mut crates_parser = separated_list1(char(' '), nom_crate);

    let (input, crates) = crates_parser(input)?;

    Ok((input, crates))
}

fn nom_all_crate_rows(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>> {
    let mut line_parser = separated_list1(line_ending, nom_crate_row);

    let (input, crates) = line_parser(input)?;
    let (input, _) = preceded(space0, line_ending)(input)?; // Get rid of that final line

    Ok((input, crates))
}

fn nom_column_count(input: &str) -> IResult<&str, usize> {
    let mut count_parser = preceded(
        multispace0,
        separated_list1(multispace0, nom_unsigned_digit::<usize>),
    );

    let (input, counts) = count_parser(input)?;
    let (input, _) = preceded(space0, line_ending)(input)?; // Get rid of that final line

    // Get the last element of counts
    Ok((input, *counts.last().unwrap()))
}

fn nom_instruction(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, _) = tag("move ")(input)?;
    let (input, crate_count) = nom_unsigned_digit::<usize>(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_stack) = nom_unsigned_digit::<usize>(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_stack) = nom_unsigned_digit::<usize>(input)?;

    Ok((input, (crate_count, from_stack, to_stack)))
}

fn nom_instructions(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    let mut instruction_parser = separated_list1(line_ending, nom_instruction);
    let (input, instructions) = instruction_parser(input)?;
    let (input, _) = preceded(space0, line_ending)(input)?; // Get rid of that final line
    Ok((input, instructions))
}

/// Parse the input for the puzzle
///
/// The input will be parsed into a tuple of two elements. The first element is a list of stacks,
/// The second element element is a list of instructions.
pub fn parse_input(input: &str) -> Result<(Vec<Stack>, Vec<Instruction>)> {
    // Parse the crate rows
    let (input, crate_rows) =
        nom_all_crate_rows(input).map_err(|e| error!("Unable to parse: {}", e))?;
    // Parse the column count
    let (input, column_count) =
        nom_column_count(input).map_err(|e| error!("Unable to parse: {}", e))?;
    // There's an empty line before instructions
    let (input, _) = preceded::<_, _, _, nom::error::Error<_>, _, _>(space0, line_ending)(input)
        .map_err(|e| error!("Unable to parse {}", e))?;
    // And finally all the instructions
    let (_, instructions) =
        nom_instructions(input).map_err(|e| error!("Unable to parse: {}", e))?;

    // Create the stacks
    let mut stacks: Vec<Stack> = vec![Stack { crates: vec![] }; column_count];

    // Then populate the stacks, by going through the rows in reverse order
    for row in crate_rows.iter().rev() {
        for (i, crate_) in row.iter().enumerate() {
            if let Some(crate_) = crate_ {
                stacks[i].crates.push(crate_.chars().next().unwrap());
            }
        }
    }

    // And finally just parse the instructions
    let instructions = instructions
        .into_iter()
        .map(|(count, from, to)| Instruction { count, from, to })
        .collect();

    Ok((stacks, instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_nom_crate() {
        assert_eq!(nom_crate("[A]"), Ok(("", Some("A"))));
        assert_eq!(nom_crate("   "), Ok(("", None)));
    }

    #[test]
    fn test_nom_crate_row() {
        assert_eq!(
            nom_crate_row("[A] [B]     [C] [D]"),
            Ok(("", vec![Some("A"), Some("B"), None, Some("C"), Some("D"),]))
        );
    }

    #[test]
    fn test_nom_all_crate_rows() {
        let lines = [
            "[A] [B]     [C] [D]",
            "[E] [F] [G] [H] [I]",
            " 1   2   3   4   5",
        ];
        let input = lines.join("\n");

        assert_eq!(
            nom_all_crate_rows(&input),
            Ok((
                " 1   2   3   4   5",
                vec![
                    vec![Some("A"), Some("B"), None, Some("C"), Some("D")],
                    vec![Some("E"), Some("F"), Some("G"), Some("H"), Some("I")],
                ]
            ))
        );
    }

    #[test]
    fn test_nom_column_count() {
        assert_eq!(nom_column_count(" 1   2   3   4   5\n"), Ok(("", 5)));
    }

    #[test]
    fn test_nom_instruction() {
        assert_eq!(nom_instruction("move 2 from 8 to 4"), Ok(("", (2, 8, 4))));
    }

    #[test]
    fn test_parse_input() {
        let (stacks, instructions) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0].crates.len(), 2);
        assert_eq!(instructions.len(), 4);
    }
}
