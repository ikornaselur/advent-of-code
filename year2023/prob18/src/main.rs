use advent::prelude::*;

/// An instruction is in the form of:
///
/// R 6 (#70c710)
///
/// where the first character is U/D/L/R for Up/Down/Left/Right
/// second character is number of steps
/// the third part is the hex colour code
#[derive(Debug)]
struct Instruction {
    direction: GridDirection,
    distance: i64,
}

impl Instruction {
    fn from_basic(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let direction =
            GridDirection::from_udlr(parts.next().ok_or(error!("Unable to parse direction"))?)
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
            "0" => GridDirection::Right,
            "1" => GridDirection::Down,
            "2" => GridDirection::Left,
            "3" => GridDirection::Up,
            _ => return Err(error!("Unable to parse direction")),
        };

        Ok(Self {
            direction,
            distance,
        })
    }
}

fn get_nodes(instructions: &[Instruction]) -> Vec<GridCoordinate<i64>> {
    let mut current: GridCoordinate<i64> = GridCoordinate { row: 0, column: 0 };
    let mut nodes = vec![];

    for instruction in instructions {
        let dist = instruction.distance;

        current = match instruction.direction {
            GridDirection::Up => GridCoordinate {
                row: current.row - dist,
                column: current.column,
            },
            GridDirection::Down => GridCoordinate {
                row: current.row + dist,
                column: current.column,
            },
            GridDirection::Left => GridCoordinate {
                row: current.row,
                column: current.column - dist,
            },
            GridDirection::Right => GridCoordinate {
                row: current.row,
                column: current.column + dist,
            },
            _ => panic!("Bad direction"),
        };
        nodes.push(current);
    }
    nodes
}

/// Get the area of a polygon
///
/// Using the Shoelace algorithm
fn get_polygon_area(nodes: &[GridCoordinate<i64>]) -> i64 {
    let num_of_nodes = nodes.len();
    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    let mut border: i64 = 0;

    for i in 0..(num_of_nodes - 1) {
        sum1 += nodes[i].row * nodes[i + 1].column;
        sum2 += nodes[i].column * nodes[i + 1].row;
        border +=
            (nodes[i].row - nodes[i + 1].row).abs() + (nodes[i].column - nodes[i + 1].column).abs();
    }

    sum1 += nodes[num_of_nodes - 1].row * nodes[0].column;
    sum2 += nodes[num_of_nodes - 1].column * nodes[0].row;
    border += (nodes[num_of_nodes - 1].row - nodes[0].row).abs()
        + (nodes[num_of_nodes - 1].column - nodes[0].column).abs();

    let inside = (sum1 - sum2).abs() / 2;

    // Note: Full disclosure, I have no idea why this works, I just noticed that the number I was
    // getting from shoelace algorithm was less then expected in example.. and counting the border
    // was going way over, then half of the border (half outside and half inside the shoelace?)
    // was one off ... soooo +1 and that matched the test case plus part 1 (which I solved first
    // with flood fill), so here we are.
    inside + (border / 2) + 1
}

fn main() -> Result<()> {
    let input = get_input(2023, 18)?;

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

        assert_eq!(instruction.direction, GridDirection::Right);
        assert_eq!(instruction.distance, 16);
    }

    #[test]
    fn test_parse_instruction_hex() {
        let instruction = Instruction::from_hex("R 16 (#0dc571)").unwrap();

        assert_eq!(instruction.direction, GridDirection::Down);
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
        assert_eq!(nodes[0], GridCoordinate { row: 0, column: 8 });
        assert_eq!(nodes[1], GridCoordinate { row: 4, column: 8 });
        assert_eq!(nodes[2], GridCoordinate { row: 4, column: 0 });
        assert_eq!(nodes[3], GridCoordinate { row: 0, column: 0 });
    }
}
