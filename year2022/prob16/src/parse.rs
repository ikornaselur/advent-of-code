use advent::prelude::*;

use crate::Valve;

type Line = ((char, char), usize, Vec<(char, char)>);

fn nom_valve(input: &str) -> IResult<&str, (char, char)> {
    let is_uppercase = |c: char| c.is_ascii_uppercase();

    tuple((satisfy(is_uppercase), satisfy(is_uppercase)))(input)
}

fn nom_tunnels(input: &str) -> IResult<&str, Vec<(char, char)>> {
    preceded(
        alt((
            tag("tunnels lead to valves "),
            tag("tunnel leads to valve "),
        )),
        separated_list1(tag(", "), nom_valve),
    )(input)
}

fn nom_full_line(input: &str) -> IResult<&str, Line> {
    let (input, valve) = preceded(tag("Valve "), nom_valve)(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow) = terminated(map_res(digit1, |s: &str| s.parse()), tag(";"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, tunnels) = nom_tunnels(input)?;

    Ok((input, (valve, flow, tunnels)))
}

pub fn parse_input(input: &str) -> Result<Vec<Valve>> {
    let (_, valves) = separated_list1(newline, nom_full_line)(input)
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(valves
        .into_iter()
        .enumerate()
        .map(|(id, (name, flow_rate, tunnels))| Valve {
            id,
            name,
            flow_rate,
            tunnels,
            tunnel_ids: None,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mom_valve() {
        assert_eq!(nom_valve("AA"), Ok(("", ('A', 'A'))));
    }

    #[test]
    fn test_nom_tunnels() {
        assert_eq!(
            nom_tunnels("tunnels lead to valves AA, BB, CD"),
            Ok(("", vec![('A', 'A'), ('B', 'B'), ('C', 'D')]))
        );
        assert_eq!(
            nom_tunnels("tunnel leads to valve AA"),
            Ok(("", vec![('A', 'A')]))
        );
    }

    #[test]
    fn test_nom_full_line() {
        assert_eq!(
            nom_full_line("Valve AA has flow rate=1; tunnel leads to valve BB"),
            Ok(("", (('A', 'A'), 1, vec![('B', 'B')])))
        );
        assert_eq!(
            nom_full_line("Valve AA has flow rate=1; tunnels lead to valves BB, CC"),
            Ok(("", (('A', 'A'), 1, vec![('B', 'B'), ('C', 'C')])))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = [
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
        ]
        .join("\n");

        let result = parse_input(&input).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            Valve {
                id: 0,
                name: ('G', 'G'),
                flow_rate: 0,
                tunnels: vec![('F', 'F'), ('H', 'H')],
                tunnel_ids: None,
            }
        );
        assert_eq!(
            result[1],
            Valve {
                id: 1,
                name: ('H', 'H'),
                flow_rate: 22,
                tunnels: vec![('G', 'G')],
                tunnel_ids: None,
            }
        );
    }
}
