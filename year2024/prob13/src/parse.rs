use crate::{Button, Problem};
use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

fn nom_button(input: &str) -> IResult<&str, Button> {
    let (input, button_id) = preceded(tag("Button "), one_of("AB")).parse(input)?;
    let (input, x) = preceded(tag(": X+"), nom_unsigned_digit).parse(input)?;
    let (input, y) = preceded(tag(", Y+"), nom_unsigned_digit).parse(input)?;

    let button = match button_id {
        'A' => Button { x, y },
        'B' => Button { x, y },
        _ => unreachable!(),
    };

    Ok((input, button))
}

fn nom_prize(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, x) = preceded(tag("Prize: X="), nom_unsigned_digit).parse(input)?;
    let (input, y) = preceded(tag(", Y="), nom_unsigned_digit).parse(input)?;

    Ok((input, (x, y)))
}

fn nom_problem(input: &str) -> IResult<&str, Problem> {
    let (input, a) = nom_button(input)?;
    let (input, b) = preceded(newline, nom_button).parse(input)?;
    let (input, prize) = preceded(newline, nom_prize).parse(input)?;

    Ok((input, Problem { a, b, prize }))
}

pub fn parse_input(input: &str) -> Result<Vec<Problem>> {
    let (_, problems) = separated_list1(many1(newline), nom_problem)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_button() {
        assert_eq!(
            nom_button("Button A: X+1, Y+2"),
            Ok(("", (Button { x: 1, y: 2 })))
        );
        assert_eq!(
            nom_button("Button B: X+3, Y+4"),
            Ok(("", (Button { x: 3, y: 4 })))
        );
    }

    #[test]
    fn test_nom_prize() {
        assert_eq!(nom_prize("Prize: X=1, Y=2"), Ok(("", (1, 2))));
    }

    #[test]
    fn test_nom_problem() {
        let input = [
            "Button A: X+1, Y+2",
            "Button B: X+3, Y+4",
            "Prize: X=5, Y=6",
        ]
        .join("\n");
        let output = Problem {
            a: Button { x: 1, y: 2 },
            b: Button { x: 3, y: 4 },
            prize: (5, 6),
        };
        assert_eq!(nom_problem(&input), Ok(("", output)));
    }

    #[test]
    fn test_parse_input() {
        let input = [
            "Button A: X+1, Y+2",
            "Button B: X+3, Y+4",
            "Prize: X=5, Y=6",
            "",
            "Button A: X+7, Y+8",
            "Button B: X+9, Y+10",
            "Prize: X=11, Y=12",
        ]
        .join("\n");

        let output = [
            Problem {
                a: Button { x: 1, y: 2 },
                b: Button { x: 3, y: 4 },
                prize: (5, 6),
            },
            Problem {
                a: Button { x: 7, y: 8 },
                b: Button { x: 9, y: 10 },
                prize: (11, 12),
            },
        ];

        assert_eq!(parse_input(&input).unwrap(), output);
    }
}
