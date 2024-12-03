use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

use crate::Packet;

/// Parse a list of packets, which can be either usize values or list of them. The nesting can be
/// multiple levels.
///
/// Examples of inputs:
///     * [1,2,3]
///     * [[1],2,3]
///     * [[[1],2],3]
///     * []
///     * [[[]]]
fn nom_packets(input: &str) -> IResult<&str, Packet> {
    alt((
        map(nom_unsigned_digit::<usize>, Packet::Value),
        map(
            delimited(
                char('['),
                separated_list0(char(','), nom_packets),
                char(']'),
            ),
            Packet::Group,
        ),
    ))(input)
}

fn nom_packets_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    terminated(
        separated_pair(nom_packets, newline, nom_packets),
        opt(newline),
    )(input)
}

pub fn parse_packets(input: &str) -> Result<Vec<(Packet, Packet)>> {
    let mut parser = separated_list1(newline, nom_packets_pair);
    let (_, packets) = parser(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse heightmap: {:?}", e)))?;

    Ok(packets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_packets_base_case() {
        let input = "[1,2,3]";
        assert_eq!(
            nom_packets(input).unwrap(),
            (
                "",
                Packet::Group(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)])
            )
        );
    }

    #[test]
    fn test_nom_packets_nested_case() {
        let input = "[[1],2,3]";
        assert_eq!(
            nom_packets(input).unwrap(),
            (
                "",
                Packet::Group(vec![
                    Packet::Group(vec![Packet::Value(1)]),
                    Packet::Value(2),
                    Packet::Value(3)
                ])
            )
        );
    }

    #[test]
    fn test_nom_packets_multiple_nested_case() {
        let input = "[[[1],2],3]";
        assert_eq!(
            nom_packets(input).unwrap(),
            (
                "",
                Packet::Group(vec![
                    Packet::Group(vec![
                        Packet::Group(vec![Packet::Value(1)]),
                        Packet::Value(2)
                    ]),
                    Packet::Value(3)
                ])
            )
        );
    }

    #[test]
    fn test_nom_packets_empty_case() {
        let input = "[]";
        assert_eq!(nom_packets(input).unwrap(), ("", Packet::Group(vec![])));
    }

    #[test]
    fn test_nom_packets_multiple_nested_empty_case() {
        let input = "[[[]]]";
        assert_eq!(
            nom_packets(input).unwrap(),
            (
                "",
                Packet::Group(vec![Packet::Group(vec![Packet::Group(vec![])])])
            )
        );
    }

    #[test]
    fn test_nom_packets_pair() {
        let input = "[1,2,3]\n[[4],5,6]\n";
        assert_eq!(
            nom_packets_pair(input).unwrap(),
            (
                "",
                (
                    Packet::Group(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]),
                    Packet::Group(vec![
                        Packet::Group(vec![Packet::Value(4)]),
                        Packet::Value(5),
                        Packet::Value(6)
                    ])
                )
            )
        );
    }

    #[test]
    fn test_parse_packets() {
        let input = "[1,2,3]\n[[4],5,6]\n\n[]\n[5]";
        assert_eq!(
            parse_packets(input).unwrap(),
            vec![
                (
                    Packet::Group(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]),
                    Packet::Group(vec![
                        Packet::Group(vec![Packet::Value(4)]),
                        Packet::Value(5),
                        Packet::Value(6)
                    ])
                ),
                (Packet::Group(vec![]), Packet::Group(vec![Packet::Value(5)])),
            ]
        );
    }
}
