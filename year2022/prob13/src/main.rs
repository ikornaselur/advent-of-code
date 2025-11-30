use advent::prelude::*;
use parse::parse_packets;
use std::cmp::Ordering;
use std::fmt;

mod parse;

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Value(usize),
    Group(Vec<Packet>),
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Value(val) => write!(f, "{}", val),
            Packet::Group(packets) => {
                let formatted_packets: Vec<String> =
                    packets.iter().map(|p| format!("{}", p)).collect();
                write!(f, "[{}]", formatted_packets.join(", "))
            }
        }
    }
}

fn main() -> Result<()> {
    let input = get_input(2022, 13)?;

    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, &input)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, &input)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        &input,
    );

    Ok(())
}

/// Compare packets based on the following rules:
///
/// 1. If both values are integers, if left is lower, then return true
/// 2. If both values are lists, compare the first values, then second and so on.
///    a. If left list runs out of items first, then return true
/// 3. If only one value is a list, convert the other value into a list of that single value and
///    continue as normal
///
/// Continue while values are the same, but return immediately if either we find a valid or
/// invalid value
fn compare_packets(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Packet::Value(left), Packet::Value(right)) => {
            if left == right {
                None
            } else {
                Some(left < right)
            }
        }
        (Packet::Group(left), Packet::Group(right)) => {
            for i in 0..left.len() {
                if i >= right.len() {
                    return Some(false);
                }
                if let Some(result) = compare_packets(&left[i], &right[i]) {
                    return Some(result);
                }
            }
            if left.len() < right.len() {
                Some(true)
            } else {
                None
            }
        }
        (Packet::Group(left), Packet::Value(right)) => compare_packets(
            &Packet::Group(left.clone()),
            &Packet::Group(vec![Packet::Value(*right)]),
        ),
        (Packet::Value(left), Packet::Group(right)) => compare_packets(
            &Packet::Group(vec![Packet::Value(*left)]),
            &Packet::Group(right.clone()),
        ),
    }
}

fn part1(input: &str) -> Result<usize> {
    let packet_pairs = parse_packets(input)?;

    Ok(packet_pairs
        .iter()
        .enumerate()
        .map(|(idx, (left, right))| {
            if let Some(true) = compare_packets(left, right) {
                idx + 1
            } else {
                0
            }
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let packet_pairs = parse_packets(input)?;

    // Flatten the list of tuples into a list of packets
    let mut packets = packet_pairs
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .collect::<Vec<_>>();

    let group_2 = Packet::Group(vec![Packet::Group(vec![Packet::Value(2)])]);
    let group_6 = Packet::Group(vec![Packet::Group(vec![Packet::Value(6)])]);

    // We then add the two new packets from part 2
    packets.push(&group_2);
    packets.push(&group_6);

    // Now we sort the list, following the rules from 'compare_packets'
    packets.sort_unstable_by(|left, right| {
        compare_packets(left, right)
            .map(|result| {
                if result {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_or(Ordering::Equal)
    });

    // Now find the inde of the two new packets
    let index_2 = packets.iter().position(|p| *p == &group_2).unwrap();
    let index_6 = packets.iter().position(|p| *p == &group_6).unwrap();

    Ok((index_2 + 1) * (index_6 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 140);
    }

    #[test]
    fn test_compare_packets_values() {
        // If left is lower, we have a valid output
        assert_eq!(
            compare_packets(&Packet::Value(1), &Packet::Value(2)),
            Some(true)
        );

        // If left is higher, we have an invalid output
        assert_eq!(
            compare_packets(&Packet::Value(2), &Packet::Value(1)),
            Some(false)
        );

        // If both are the same, we have an inconclusive output
        assert_eq!(compare_packets(&Packet::Value(2), &Packet::Value(2)), None);
    }

    #[test]
    fn test_compare_packets_lists() {
        // Empty lists are inconclusive
        assert_eq!(
            compare_packets(&Packet::Group(vec![]), &Packet::Group(vec![])),
            None,
        );

        // Left list is shorter, which is fine
        assert_eq!(
            compare_packets(
                &Packet::Group(vec![]),
                &Packet::Group(vec![Packet::Value(1)]),
            ),
            Some(true)
        );

        // Right list is shorter, which is not fine
        assert_eq!(
            compare_packets(
                &Packet::Group(vec![Packet::Value(1)]),
                &Packet::Group(vec![]),
            ),
            Some(false)
        );
    }
}
