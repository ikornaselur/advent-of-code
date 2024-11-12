use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node(String);

impl Node {
    fn is_start(&self) -> bool {
        self.0.ends_with('A')
    }

    fn is_end(&self) -> bool {
        self.0.ends_with('Z')
    }
}

enum Direction {
    R,
    L,
}

struct Map {
    nodes: HashMap<Node, (Node, Node)>,
    directions: Vec<Direction>,
}

impl FromStr for Map {
    type Err = AdventError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = input.lines();
        // First line is the directions, which is a list of R and L (no spacing)
        let directions = lines
            .next()
            .ok_or(AdventError::ParseError("No directions found".to_string()))?
            .chars()
            .map(|c| match c {
                'R' => Direction::R,
                'L' => Direction::L,
                _ => panic!("Unknown direction"),
            })
            .collect();

        // Followed by an empty line
        lines.next();

        // And then all following lines show a mapping in the form of
        //
        //   AAA = (BBB, CCC)
        //
        // which means that AAA goes left to BBB or right to CCC, we'll just represent this as a
        // hashmap to the tuple
        let nodes = lines
            .map(|line| {
                let mut parts = line.split(" = ");
                let node = parts.next().ok_or(AdventError::ParseError(
                    "No node found in mapping".to_string(),
                ))?;
                let mapping = parts
                    .next()
                    .ok_or(AdventError::ParseError(
                        "No mapping found in mapping".to_string(),
                    ))?
                    .strip_prefix('(')
                    .ok_or(AdventError::ParseError(
                        "Mapping does not start with (".to_string(),
                    ))?
                    .strip_suffix(')')
                    .ok_or(AdventError::ParseError(
                        "Mapping does not end with )".to_string(),
                    ))?;
                let mut mapping_parts = mapping.split(", ");
                let left = mapping_parts.next().ok_or(AdventError::ParseError(
                    "No left node found in mapping".to_string(),
                ))?;
                let right = mapping_parts.next().ok_or(AdventError::ParseError(
                    "No right node found in mapping".to_string(),
                ))?;
                Ok((
                    Node(node.to_string()),
                    (Node(left.to_string()), Node(right.to_string())),
                ))
            })
            .collect::<Result<HashMap<Node, (Node, Node)>>>()?;

        Ok(Self { nodes, directions })
    }
}

fn calculate_lcm(numbers: &[u64]) -> u64 {
    // Return early if the list is empty
    if numbers.is_empty() {
        return 0;
    }

    // Find the LCM of two numbers
    fn lcm_of_two(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    // Find the greatest common divisor (GCD) of two numbers
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    // Use the LCM of two numbers iteratively for the entire list
    numbers.iter().cloned().fold(1, lcm_of_two)
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;

    // We'll follow the directions until we reach the end, which is ZZZ
    let mut current_node = Node("AAA".to_string());
    let mut steps_taken = 0;
    loop {
        let (left, right) =
            map.nodes
                .get(&current_node)
                .ok_or(AdventError::ParseError(format!(
                    "No node found for {}",
                    current_node.0
                )))?;
        let steps_idx = steps_taken % map.directions.len();
        match map.directions[steps_idx] {
            Direction::L => current_node = Node(left.0.clone()),
            Direction::R => current_node = Node(right.0.clone()),
        };
        steps_taken += 1;
        if current_node == Node("ZZZ".to_string()) {
            break;
        }
    }

    Ok(steps_taken)
}

fn part2(input: &str) -> Result<u64> {
    let map = Map::from_str(input)?;

    // Get all start nodes
    let start_nodes = map
        .nodes
        .iter()
        .filter(|(node, _)| node.is_start())
        .map(|(node, _)| node)
        .collect::<Vec<&Node>>();
    let mut distances = Vec::new();

    // Get the individual nodes distances to an end
    for node in start_nodes {
        let mut current_node = Node(node.0.clone());
        let mut steps_taken = 0;
        loop {
            let (left, right) =
                map.nodes
                    .get(&current_node)
                    .ok_or(AdventError::ParseError(format!(
                        "No node found for {}",
                        current_node.0
                    )))?;
            let steps_idx = steps_taken % map.directions.len();
            match map.directions[steps_idx] {
                Direction::L => current_node = Node(left.0.clone()),
                Direction::R => current_node = Node(right.0.clone()),
            };
            steps_taken += 1;
            if current_node.is_end() {
                break;
            }
        }
        distances.push(steps_taken as u64);
    }

    // Calculate the least common multiple of all the distances
    let lcm = calculate_lcm(&distances);

    Ok(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 6);
    }

    #[test]
    fn test_map_from_str() {
        let map = Map::from_str(PART_1_TEST_INPUT).unwrap();
        assert_eq!(map.directions.len(), 3);
        assert_eq!(map.nodes.len(), 3);
    }

    #[test]
    fn test_node_is_start() {
        assert!(Node("AAA".to_string()).is_start());
        assert!(Node("CBA".to_string()).is_start());
        assert!(!Node("ABC".to_string()).is_start());
    }

    #[test]
    fn test_node_is_end() {
        assert!(Node("ZZZ".to_string()).is_end());
        assert!(Node("XYZ".to_string()).is_end());
        assert!(!Node("ABC".to_string()).is_end());
    }
}
