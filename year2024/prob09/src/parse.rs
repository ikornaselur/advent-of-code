use crate::CompressedNode;
use advent::prelude::*;

fn nom_digit(input: &str) -> IResult<&str, u32> {
    map(one_of("0123456789"), |d: char| d.to_digit(10).unwrap()).parse(input)
}

pub fn parse_input(input: &str) -> Result<Vec<CompressedNode>> {
    let (_, digits) = many1(nom_digit)
        .parse(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    // We alternate between a file node and a free node
    let nodes = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 0 {
                CompressedNode::File(d)
            } else {
                CompressedNode::Free(d)
            }
        })
        .collect();

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "12345";
        let output = parse_input(input).unwrap();

        assert_eq!(
            output,
            vec![
                CompressedNode::File(1),
                CompressedNode::Free(2),
                CompressedNode::File(3),
                CompressedNode::Free(4),
                CompressedNode::File(5)
            ]
        );
    }
}
