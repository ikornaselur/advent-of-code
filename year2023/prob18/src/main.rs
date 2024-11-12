use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

/// An instruction is in the form of:
///
/// R 6 (#70c710)
///
/// where the first character is U/D/L/R for Up/Down/Left/Right
/// second character is number of steps
/// the third part is the hex colour code
#[derive(Debug)]
struct Instruction {
    direction: OrdinalDirection,
    distance: i64,
}

impl Instruction {
    fn from_basic(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let direction =
            OrdinalDirection::from_udlr(parts.next().ok_or(error!("Unable to parse direction"))?)
                .ok_or(error!("Unable to parse direction"))?;
        let distance = parts
            .next()
            .ok_or(error!("Unable to parse steps"))?
            .parse()?;

        Ok(Self {
            direction,
            distance,
        })
    }

    fn from_hex(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        parts.next(); // Skip the basic direction
        parts.next(); // Skip the basic distance

        let colour: String = parts
            .next()
            .ok_or(error!("Unable to parse colour"))?
            .parse()
            .unwrap();

        // colour at this point is in the form: (#aaaaab)
        // where a is a 5 hex digit number for distance and
        // b is a direction where:
        //  * 0 = R
        //  * 1 = D
        //  * 2 = L
        //  * 3 = U
        let distance = i64::from_str_radix(&colour[2..7], 16)?;
        let direction = match &colour[7..8] {
            "0" => OrdinalDirection::Right,
            "1" => OrdinalDirection::Down,
            "2" => OrdinalDirection::Left,
            "3" => OrdinalDirection::Up,
            _ => return Err(error!("Unable to parse direction")),
        };

        Ok(Self {
            direction,
            distance,
        })
    }
}

fn get_nodes(instructions: &[Instruction]) -> Vec<Coordinate<i64>> {
    let mut current: Coordinate<i64> = (0, 0);
    let mut nodes = vec![];

    for instruction in instructions {
        let dist = instruction.distance;

        current = match instruction.direction {
            OrdinalDirection::Up => (current.0 - dist, current.1),
            OrdinalDirection::Down => (current.0 + dist, current.1),
            OrdinalDirection::Left => (current.0, current.1 - dist),
            OrdinalDirection::Right => (current.0, current.1 + dist),
        };
        nodes.push(current);
    }
    nodes
}

/// Get the area of a polygon
///
/// Using the Shoelace algorithm
fn get_polygon_area(nodes: &[Coordinate<i64>]) -> i64 {
    let num_of_nodes = nodes.len();
    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    let mut border: i64 = 0;

    for i in 0..(num_of_nodes - 1) {
        sum1 += nodes[i].0 * nodes[i + 1].1;
        sum2 += nodes[i].1 * nodes[i + 1].0;
        border += (nodes[i].0 - nodes[i + 1].0).abs() + (nodes[i].1 - nodes[i + 1].1).abs();
    }

    sum1 += nodes[num_of_nodes - 1].0 * nodes[0].1;
    sum2 += nodes[num_of_nodes - 1].1 * nodes[0].0;
    border += (nodes[num_of_nodes - 1].0 - nodes[0].0).abs()
        + (nodes[num_of_nodes - 1].1 - nodes[0].1).abs();

    let inside = (sum1 - sum2).abs() / 2;

    // Note: Full disclosure, I have no idea why this works, I just noticed that the number I was
    // getting from shoelace algorithm was less then expected in example.. and counting the border
    // was going way over, then half of the border (half outside and half inside the shoelace?)
    // was one off ... soooo +1 and that matched the test case plus part 1 (which I solved first
    // with flood fill), so here we are.
    inside + (border / 2) + 1
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<i64> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(Instruction::from_basic)
        .collect::<Result<Vec<Instruction>>>()?;
    let nodes = get_nodes(&instructions);

    Ok(get_polygon_area(&nodes))
}

fn part2(input: &str) -> Result<i64> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(Instruction::from_hex)
        .collect::<Result<Vec<Instruction>>>()?;
    let nodes = get_nodes(&instructions);

    Ok(get_polygon_area(&nodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 952408144115);
    }

    #[test]
    fn test_parse_instruction_basic() {
        let instruction = Instruction::from_basic("R 16 (#70c710)").unwrap();

        assert_eq!(instruction.direction, OrdinalDirection::Right);
        assert_eq!(instruction.distance, 16);
    }

    #[test]
    fn test_parse_instruction_hex() {
        let instruction = Instruction::from_hex("R 16 (#0dc571)").unwrap();

        assert_eq!(instruction.direction, OrdinalDirection::Down);
        assert_eq!(instruction.distance, 56407);
    }

    #[test]
    fn test_get_nodes() {
        let instructions: Vec<Instruction> = vec![
            Instruction::from_basic("R 8 (#70c710)").unwrap(),
            Instruction::from_basic("D 4 (#70c710)").unwrap(),
            Instruction::from_basic("L 8 (#70c710)").unwrap(),
            Instruction::from_basic("U 4 (#70c710)").unwrap(),
        ];

        let nodes = get_nodes(&instructions);

        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0], (0, 8));
        assert_eq!(nodes[1], (4, 8));
        assert_eq!(nodes[2], (4, 0));
        assert_eq!(nodes[3], (0, 0));
    }
}
