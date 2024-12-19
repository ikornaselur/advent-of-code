use crate::{Colour, Pattern};
use advent::prelude::*;

fn nom_stripes(input: &str) -> IResult<&str, Pattern> {
    many1(alt((
        value(Colour::White, char('w')),
        value(Colour::Black, char('b')),
        value(Colour::Green, char('g')),
        value(Colour::Red, char('r')),
        value(Colour::Blue, char('u')),
    )))(input)
}

fn nom_towels(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(tag(", "), nom_stripes)(input)
}

fn nom_patterns(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(newline, nom_stripes)(input)
}

pub fn parse_input(input: &str) -> Result<(Vec<Pattern>, Vec<Pattern>)> {
    let (_, (towels, patterns)) = separated_pair(nom_towels, tag("\n\n"), nom_patterns)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok((towels, patterns))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_stripes() {
        assert_eq!(nom_stripes("w"), Ok(("", vec![Colour::White])));
        assert_eq!(
            nom_stripes("wbu"),
            Ok(("", vec![Colour::White, Colour::Black, Colour::Blue]))
        );
    }

    #[test]
    fn test_nom_towels() {
        assert_eq!(
            nom_towels("w, wbu"),
            Ok((
                "",
                vec![
                    vec![Colour::White],
                    vec![Colour::White, Colour::Black, Colour::Blue]
                ]
            ))
        );
    }

    #[test]
    fn test_nom_patterns() {
        assert_eq!(
            nom_patterns("w\nwbu"),
            Ok((
                "",
                vec![
                    vec![Colour::White],
                    vec![Colour::White, Colour::Black, Colour::Blue]
                ]
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "w, wbu\n\nw\nwbu";
        assert_eq!(
            parse_input(input).unwrap(),
            (
                vec![
                    vec![Colour::White],
                    vec![Colour::White, Colour::Black, Colour::Blue]
                ],
                vec![
                    vec![Colour::White],
                    vec![Colour::White, Colour::Black, Colour::Blue]
                ]
            )
        );
    }
}
