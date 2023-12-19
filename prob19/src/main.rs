use advent::prelude::*;
use regex::Regex;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    /// Run the part through a workflow
    ///
    /// The output of the workflow will indicate the next step for the part
    fn run_workflow(&self, workflow: &Workflow) -> String {
        for condition in &workflow.conditions {
            match condition {
                Condition {
                    category: None,
                    value: None,
                    ordering: None,
                    workflow_name,
                } => {
                    // If there is no category, value, or ordering, then this is the final step
                    return workflow_name.clone();
                }
                Condition {
                    category: Some(category),
                    value: Some(value),
                    ordering: Some(ordering),
                    workflow_name,
                } => {
                    let part_value = match category {
                        Category::X => self.x,
                        Category::M => self.m,
                        Category::A => self.a,
                        Category::S => self.s,
                    };

                    let result = match ordering {
                        Ordering::Greater => &part_value > value,
                        Ordering::Less => &part_value < value,
                        _ => panic!("Unknown ordering"),
                    };
                    if result {
                        return workflow_name.clone();
                    }
                }
                _ => panic!("Unknown condition"),
            }
        }
        panic!("Unable to find condition");
    }
}

impl FromStr for Part {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}")
            .map_err(|e| error!("Unable to create regex: {}", e))?;

        let caps = re.captures(s).ok_or(error!("Unable to get captures"))?;
        let x = caps
            .name("x")
            .ok_or(error!("Unable to get x"))?
            .as_str()
            .parse::<u32>()?;
        let m = caps
            .name("m")
            .ok_or(error!("Unable to get m"))?
            .as_str()
            .parse::<u32>()?;
        let a = caps
            .name("a")
            .ok_or(error!("Unable to get a"))?
            .as_str()
            .parse::<u32>()?;
        let s = caps
            .name("s")
            .ok_or(error!("Unable to get s"))?
            .as_str()
            .parse::<u32>()?;

        Ok(Part { x, m, a, s })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
}

impl FromStr for Workflow {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"(?<name>\w+)\{(?<conditions>.+)\}")
            .map_err(|e| error!("Unable to create regex: {}", e))?;

        let caps = re.captures(s).ok_or(error!("Unable to get captures"))?;
        let name = caps
            .name("name")
            .ok_or(error!("Unable to get name"))?
            .as_str()
            .to_string();

        let conditions = caps
            .name("conditions")
            .ok_or(error!("Unable to get conditions"))?
            .as_str()
            .split(',')
            .map(|s| s.parse::<Condition>())
            .collect::<Result<Vec<Condition>>>()?;

        Ok(Workflow { name, conditions })
    }
}

#[derive(Debug, PartialEq)]
struct Condition {
    category: Option<Category>,
    value: Option<u32>,
    ordering: Option<Ordering>,
    workflow_name: String,
}

impl FromStr for Condition {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // A condition string will look like this:
        //
        //   a<2006:qkq
        //
        // This is <category><operation><value>:<workflow>
        let re =
            Regex::new(r"(?:(?P<category>\w)(?P<ordering>[<>])(?P<value>\d+):)?(?P<workflow>\w+)")
                .map_err(|e| error!("Unable to create regex: {}", e))?;
        let caps = re.captures(s).ok_or(error!("Unable to get captures"))?;

        let category = match caps.name("category") {
            Some(category) => match category.as_str() {
                "x" => Some(Category::X),
                "m" => Some(Category::M),
                "a" => Some(Category::A),
                "s" => Some(Category::S),
                _ => return Err(error!("Unknown category")),
            },
            None => None,
        };

        let value = match caps.name("value") {
            Some(value) => Some(value.as_str().parse::<u32>()?),
            None => None,
        };

        let ordering = match caps.name("ordering") {
            Some(ordering) => match ordering.as_str() {
                ">" => Some(Ordering::Greater),
                "<" => Some(Ordering::Less),
                _ => return Err(error!("Unknown ordering")),
            },
            None => None,
        };

        let workflow_name = caps
            .name("workflow")
            .ok_or(error!("Unable to get workflow"))?
            .as_str()
            .to_string();

        Ok(Condition {
            category,
            value,
            ordering,
            workflow_name,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Ranges {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl Ranges {
    fn combinations(&self) -> u64 {
        (self.x.1 - self.x.0) as u64
            * (self.m.1 - self.m.0) as u64
            * (self.a.1 - self.a.0) as u64
            * (self.s.1 - self.s.0) as u64
    }
}

fn get_combinations(key: &str, workflows: &HashMap<String, Workflow>, ranges: &Ranges) -> u64 {
    if key == "A" {
        return ranges.combinations();
    }
    if key == "R" {
        return 0;
    }

    let workflow = workflows.get(key).unwrap();
    let mut combinations = 0;

    let mut ranges = *ranges;

    // Loop through the conditions
    //
    // For any condition that has a category, value and ordering, we split up the ranges and recurse
    // For the other half of the range, we continue through to the next condition
    //
    // When we reach the final condition (which will not have a category, value or ordering), we
    // recurse without splitting the ranges any more
    for condition in &workflow.conditions {
        match condition {
            Condition {
                category: None,
                value: None,
                ordering: None,
                workflow_name,
            } => {
                // This is the final condition, we will just recurse with the ranges as they are
                // now
                combinations += get_combinations(workflow_name, workflows, &ranges);
            }
            Condition {
                category: Some(category),
                value: Some(value),
                ordering: Some(ordering),
                workflow_name,
            } => {
                // We need to split up the ranges and recurse
                let mut new_ranges = ranges;
                match (category, ordering) {
                    (Category::X, Ordering::Greater) => {
                        new_ranges.x = (value + 1, ranges.x.1);
                        ranges.x = (ranges.x.0, value + 1);
                    }
                    (Category::X, Ordering::Less) => {
                        new_ranges.x = (ranges.x.0, *value);
                        ranges.x = (*value, ranges.x.1);
                    }
                    (Category::M, Ordering::Greater) => {
                        new_ranges.m = (value + 1, ranges.m.1);
                        ranges.m = (ranges.m.0, value + 1);
                    }
                    (Category::M, Ordering::Less) => {
                        new_ranges.m = (ranges.m.0, *value);
                        ranges.m = (*value, ranges.m.1);
                    }
                    (Category::A, Ordering::Greater) => {
                        new_ranges.a = (value + 1, ranges.a.1);
                        ranges.a = (ranges.a.0, value + 1);
                    }
                    (Category::A, Ordering::Less) => {
                        new_ranges.a = (ranges.a.0, *value);
                        ranges.a = (*value, ranges.a.1);
                    }
                    (Category::S, Ordering::Greater) => {
                        new_ranges.s = (value + 1, ranges.s.1);
                        ranges.s = (ranges.s.0, value + 1);
                    }
                    (Category::S, Ordering::Less) => {
                        new_ranges.s = (ranges.s.0, *value);
                        ranges.s = (*value, ranges.s.1);
                    }
                    _ => panic!("Unknown condition"),
                }
                combinations += get_combinations(workflow_name, workflows, &new_ranges);
            }
            _ => panic!("Unknown condition"),
        }
    }

    combinations
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    // Input will be multiple lines of workflows, followed by multiple lines of parts. The two
    // sections will be split by an empty line
    let mut sections = input.split("\n\n");
    let workflows: HashMap<String, Workflow> = sections
        .next()
        .ok_or(error!("Unable to get workflows"))?
        .lines()
        .map(|s| {
            let workflow = s.parse::<Workflow>()?;
            Ok((workflow.name.clone(), workflow))
        })
        .collect::<Result<HashMap<String, Workflow>>>()?;

    let parts = sections
        .next()
        .ok_or(error!("Unable to get parts"))?
        .lines()
        .map(|s| s.parse::<Part>())
        .collect::<Result<Vec<Part>>>()?;

    let total = parts
        .iter()
        .map(|part| {
            let mut workflow_name = "in".to_string();
            loop {
                let workflow = workflows
                    .get(&workflow_name)
                    .ok_or(error!("Unable to get workflow"))
                    .unwrap();
                workflow_name = part.run_workflow(workflow);
                if workflow_name == "A" {
                    return part.x + part.m + part.a + part.s;
                } else if workflow_name == "R" {
                    return 0;
                }
            }
        })
        .sum::<u32>();

    Ok(total)
}

fn part2(input: &str) -> Result<u64> {
    let mut sections = input.split("\n\n");
    let workflows: HashMap<String, Workflow> = sections
        .next()
        .ok_or(error!("Unable to get workflows"))?
        .lines()
        .map(|s| {
            let workflow = s.parse::<Workflow>()?;
            Ok((workflow.name.clone(), workflow))
        })
        .collect::<Result<HashMap<String, Workflow>>>()?;

    let combinations = get_combinations(
        "in",
        &workflows,
        &Ranges {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        },
    );

    Ok(combinations)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 19_114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 167_409_079_868_000);
    }

    #[test]
    fn test_condition_from_str() {
        let condition: Condition = "x>1:qkq".parse().unwrap();

        assert_eq!(condition.category, Some(Category::X));
        assert_eq!(condition.value, Some(1));
        assert_eq!(condition.ordering, Some(Ordering::Greater));
        assert_eq!(condition.workflow_name, "qkq");

        let condition: Condition = "rgb".parse().unwrap();
        assert_eq!(condition.category, None);
        assert_eq!(condition.value, None);
        assert_eq!(condition.ordering, None);
        assert_eq!(condition.workflow_name, "rgb");
    }

    #[test]
    fn test_workflow_from_str() {
        let workflow: Workflow = "qkq{x>1:qkq,m<2090:A}".parse().unwrap();

        assert_eq!(workflow.name, "qkq");
        assert_eq!(workflow.conditions.len(), 2);
        assert_eq!(
            workflow.conditions[0],
            Condition {
                category: Some(Category::X),
                value: Some(1),
                ordering: Some(Ordering::Greater),
                workflow_name: "qkq".to_string(),
            }
        );
        assert_eq!(
            workflow.conditions[1],
            Condition {
                category: Some(Category::M),
                value: Some(2090),
                ordering: Some(Ordering::Less),
                workflow_name: "A".to_string(),
            }
        );
    }

    #[test]
    fn test_part_from_str() {
        let part: Part = "{x=1,m=2090,a=15,s=9152}".parse().unwrap();

        assert_eq!(part.x, 1);
        assert_eq!(part.m, 2090);
        assert_eq!(part.a, 15);
        assert_eq!(part.s, 9152);
    }

    #[test]
    fn test_part_run_workflow() {
        let part: Part = "{x=1,m=194,a=15,s=9152}".parse().unwrap();

        let workflow: Workflow = "qkq{x>10:abc,m<2090:A}".parse().unwrap();
        assert_eq!(part.run_workflow(&workflow), "A");

        let workflow: Workflow = "qkq{x>10:abc,R}".parse().unwrap();
        assert_eq!(part.run_workflow(&workflow), "R");
    }

    #[test]
    fn test_ranges_combinations() {
        assert_eq!(
            Ranges {
                x: (1, 11),
                m: (1, 11),
                a: (1, 11),
                s: (1, 11),
            }
            .combinations(),
            10 * 10 * 10 * 10
        );

        assert_eq!(
            Ranges {
                x: (1, 11),
                m: (1, 11),
                a: (1, 11),
                s: (1, 10),
            }
            .combinations(),
            10 * 10 * 10 * 9
        );

        assert_eq!(
            Ranges {
                x: (1, 11),
                m: (1, 11),
                a: (1, 11),
                s: (2, 11),
            }
            .combinations(),
            10 * 10 * 10 * 9
        );

        assert_eq!(
            Ranges {
                x: (1, 11),
                m: (1, 11),
                a: (1, 11),
                s: (6, 11),
            }
            .combinations(),
            10 * 10 * 10 * 5
        );
    }

    #[test]
    fn test_get_combinations_base_case_rejected() {
        let combinations = get_combinations(
            "R",
            &HashMap::new(),
            &Ranges {
                x: (1, 4000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000),
            },
        );

        assert_eq!(combinations, 0);
    }

    #[test]
    fn test_get_combinations_base_case_accepted() {
        let combinations = get_combinations(
            "A",
            &HashMap::new(),
            &Ranges {
                x: (1, 11),
                m: (1, 11),
                a: (1, 11),
                s: (1, 11),
            },
        );
        assert_eq!(combinations, 10 * 10 * 10 * 10);
    }

    #[test]
    fn test_get_combinations_with_one_split_less() {
        let workflows: HashMap<String, Workflow> = [
            ("abc".to_string(), "abc{a<5:xyz,R}".parse().unwrap()),
            ("xyz".to_string(), "xyz{A}".parse().unwrap()),
        ]
        .into();

        let combinations = get_combinations(
            "abc",
            &workflows,
            &Ranges {
                x: (1, 11),
                m: (1, 11),
                a: (1, 11),
                s: (1, 11),
            },
        );

        assert_eq!(combinations, 10 * 10 * 10 * 4);
    }

    #[test]
    fn test_get_combinations_with_one_split_more() {
        let workflows: HashMap<String, Workflow> = [
            ("abc".to_string(), "abc{a>5:xyz,R}".parse().unwrap()),
            ("xyz".to_string(), "xyz{A}".parse().unwrap()),
        ]
        .into();

        let combinations = get_combinations(
            "abc",
            &workflows,
            &Ranges {
                x: (1, 11),
                m: (1, 11),
                a: (1, 11),
                s: (1, 11),
            },
        );

        assert_eq!(combinations, 10 * 10 * 10 * 5);
    }
}
