use crate::Button;
use advent::prelude::*;

fn nom_button(input: &str) -> IResult<&str, Button> {
    map(one_of("0123456789A"), |c: char| match c {
        '1' => Button::One,
        '2' => Button::Two,
        '3' => Button::Three,
        '4' => Button::Four,
        '5' => Button::Five,
        '6' => Button::Six,
        '7' => Button::Seven,
        '8' => Button::Eight,
        '9' => Button::Nine,
        '0' => Button::Zero,
        'A' => Button::A,
        _ => unreachable!(),
    })(input)
}

fn nom_code(input: &str) -> IResult<&str, Vec<Button>> {
    many1(nom_button)(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<Button>>> {
    let (_, codes) = separated_list1(newline, nom_code)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(codes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_code() {
        assert_eq!(
            nom_code("027A"),
            Ok((
                "",
                vec![Button::Zero, Button::Two, Button::Seven, Button::A]
            ))
        );
        assert_eq!(
            nom_code("321A"),
            Ok(("", vec![Button::Three, Button::Two, Button::One, Button::A]))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "123A\n456A\n789A";
        assert_eq!(
            parse_input(input).unwrap(),
            vec![
                vec![Button::One, Button::Two, Button::Three, Button::A],
                vec![Button::Four, Button::Five, Button::Six, Button::A],
                vec![Button::Seven, Button::Eight, Button::Nine, Button::A]
            ]
        );
    }
}
