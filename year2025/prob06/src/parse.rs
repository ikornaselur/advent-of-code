use crate::Symbol;
use advent::{parsers::nom_unsigned_digit, prelude::*};

fn nom_number_row(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(space0, separated_list1(space1, nom_unsigned_digit), space0).parse(input)
}

fn nom_symbol(input: &str) -> IResult<&str, Symbol> {
    alt((
        value(Symbol::Plus, char('+')),
        value(Symbol::Multiply, char('*')),
    ))
    .parse(input)
}

fn nom_symbols_row(input: &str) -> IResult<&str, Vec<Symbol>> {
    separated_list1(multispace1, nom_symbol).parse(input)
}

pub fn parse_input(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Symbol>)> {
    let (_, (number_rows, symbols_row)) = separated_pair(
        separated_list1(newline, nom_number_row),
        newline,
        nom_symbols_row,
    )
    .parse(input)?;

    Ok((number_rows, symbols_row))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_number_row() {
        assert_eq!(
            nom_number_row("  1 2   3     5   123"),
            Ok(("", vec![1, 2, 3, 5, 123]))
        );
    }

    #[test]
    fn test_nom_symbol() {
        assert_eq!(nom_symbol("+*"), Ok(("*", Symbol::Plus)));
        assert_eq!(nom_symbol("*"), Ok(("", Symbol::Multiply)));
    }

    #[test]
    fn test_nom_symbols_row() {
        assert_eq!(
            nom_symbols_row("* +   +      * +"),
            Ok((
                "",
                vec![
                    Symbol::Multiply,
                    Symbol::Plus,
                    Symbol::Plus,
                    Symbol::Multiply,
                    Symbol::Plus
                ]
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "1 2  3\n 4   5 6 \n* +   *";
        let (number_rows, symbols_row) = parse_input(input).unwrap();

        assert_eq!(number_rows, vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(
            symbols_row,
            vec![Symbol::Multiply, Symbol::Plus, Symbol::Multiply]
        );
    }
}
