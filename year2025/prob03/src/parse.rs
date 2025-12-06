use advent::prelude::*;

fn nom_single_digit(input: &str) -> IResult<&str, u32> {
    map_res(satisfy(|c| c.is_ascii_digit()), |c| {
        c.to_digit(10).ok_or(())
    })(input)
}

fn nom_digit_row(input: &str) -> IResult<&str, Vec<u32>> {
    many1(nom_single_digit)(input)
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    let (_, digit_rows) = separated_list1(newline, nom_digit_row)(input)?;

    Ok(digit_rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_single_digit() {
        assert_eq!(nom_single_digit("123"), Ok(("23", 1u32)));
    }

    #[test]
    fn test_nom_digit_row() {
        assert_eq!(nom_digit_row("123"), Ok(("", vec![1, 2, 3])));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("123\n456").unwrap(),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
    }
}
