use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

pub type PageOrder = (usize, usize);
pub type PageList = Vec<usize>;

fn nom_page_order(input: &str) -> IResult<&str, PageOrder> {
    separated_pair(nom_unsigned_digit, char('|'), nom_unsigned_digit)(input)
}

fn nom_page_list(input: &str) -> IResult<&str, PageList> {
    separated_list1(char(','), nom_unsigned_digit)(input)
}

pub fn parse_input(input: &str) -> Result<(Vec<PageOrder>, Vec<PageList>)> {
    let page_orders_parser = separated_list1(newline, nom_page_order);
    let page_lists_parser = separated_list1(newline, nom_page_list);
    let mut parser = separated_pair(page_orders_parser, tag("\n\n"), page_lists_parser);
    let (_, (page_orders, page_lists)) = parser(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok((page_orders, page_lists))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_page_order() {
        assert_eq!(nom_page_order("1|2"), Ok(("", (1, 2))));
    }

    #[test]
    fn test_nom_page_list() {
        assert_eq!(nom_page_list("1,2,41"), Ok(("", vec![1, 2, 41])))
    }

    #[test]
    fn test_parse_input() {
        let input = ["1|2", "3|4", "", "1,2,3,4", "4,3,2,1"].join("\n");
        let (page_orders, page_lists) = parse_input(&input).unwrap();
        assert_eq!(page_orders, vec![(1, 2), (3, 4)]);
        assert_eq!(page_lists, vec![vec![1, 2, 3, 4], vec![4, 3, 2, 1]]);
    }
}
