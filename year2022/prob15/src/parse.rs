use crate::{Beacon, Sensor};
use advent::prelude::*;

fn nom_coordinate(input: &str) -> IResult<&str, Coordinate<i32>> {
    // NOTE: We swap x and y here so that it's simpler to work with as row and columns in a
    // matrix, while matching the example graphics in the problem
    map(
        separated_pair(
            preceded(tag("x="), recognize(preceded(opt(char('-')), digit1))),
            tag(", "),
            preceded(tag("y="), recognize(preceded(opt(char('-')), digit1))),
        ),
        |(column, row): (&str, &str)| (row.parse().unwrap(), column.parse().unwrap()),
    )(input)
}

fn nom_sensor(input: &str) -> IResult<&str, Sensor> {
    map(preceded(tag("Sensor at "), nom_coordinate), Sensor)(input)
}

fn nom_beacon(input: &str) -> IResult<&str, Beacon> {
    map(
        preceded(tag(": closest beacon is at "), nom_coordinate),
        Beacon,
    )(input)
}

fn nom_sensor_beacon_pair(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    map(pair(nom_sensor, nom_beacon), |(sensor, beacon)| {
        (sensor, beacon)
    })(input)
}

pub fn parse_input(input: &str) -> Result<Vec<(Sensor, Beacon)>> {
    let (_, result) = separated_list1(line_ending, nom_sensor_beacon_pair)(input.trim())
        .map_err(|e| AdventError::ParseError(format!("Failed to parse input: {:?}", e)))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_coordinate() {
        assert_eq!(nom_coordinate("x=2, y=18").unwrap().1, (18, 2));
        assert_eq!(nom_coordinate("x=-2, y=15").unwrap().1, (15, -2));
    }

    #[test]
    fn test_nom_sensor() {
        assert_eq!(
            nom_sensor("Sensor at x=2, y=18").unwrap().1,
            Sensor((18, 2))
        );
    }

    #[test]
    fn test_nom_beacon() {
        assert_eq!(
            nom_beacon(": closest beacon is at x=-2, y=15").unwrap().1,
            Beacon((15, -2))
        );
    }

    #[test]
    fn test_nom_sensor_beacon_pair() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let (_, (sensor, beacon)) = nom_sensor_beacon_pair(input).unwrap();
        assert_eq!(sensor, Sensor((18, 2)));
        assert_eq!(beacon, Beacon((15, -2)));
    }

    #[test]
    fn test_parse_input() {
        let input = [
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        ]
        .join("\n");

        let result = parse_input(&input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (Sensor((18, 2)), Beacon((15, -2))));
        assert_eq!(result[1], (Sensor((16, 9)), Beacon((16, 10))));
    }

    #[test]
    fn test_parse_input_invalid() {
        let input = "Invalid input";
        assert!(parse_input(input).is_err());
    }
}
