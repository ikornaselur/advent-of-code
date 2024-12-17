use advent::prelude::*;

mod parse;

#[derive(Debug, PartialEq, Clone)]
enum OperationValue {
    Old,
    Number(usize),
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add(OperationValue),
    Multiply(OperationValue),
}
type MonkeyNum = usize;

#[derive(Debug, PartialEq)]
struct Monkey {
    num: MonkeyNum,
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    if_true: MonkeyNum,
    if_false: MonkeyNum,
    inspections: usize,
}

impl Monkey {
    fn apply_operation(&mut self, value: usize) -> usize {
        self.inspections += 1;
        match self.operation {
            Operation::Add(OperationValue::Old) => value + value,
            Operation::Add(OperationValue::Number(n)) => value + n,
            Operation::Multiply(OperationValue::Old) => value * value,
            Operation::Multiply(OperationValue::Number(n)) => value * n,
        }
    }

    /// Get the next monkey to throw to, based on the current Monkey test
    fn get_next_monkey(&self, value: usize) -> MonkeyNum {
        if value % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }

    /// Reduce worry level
    ///
    /// This is done by dividing by 3 and rounding down
    fn reduce_worry(&self, value: usize) -> usize {
        value / 3
    }
}

fn main() -> Result<()> {
    let input = get_input(2022, 11)?;

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

fn part1(input: &str) -> Result<usize> {
    let rounds = 20;
    let mut monkeys = parse::parse_monkeys(input)?;

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            // Go through each item and process them
            while let Some(value) = monkeys[i].items.pop_front() {
                let value = monkeys[i].apply_operation(value);
                let value = monkeys[i].reduce_worry(value);
                let next_monkey = monkeys[i].get_next_monkey(value);
                monkeys[next_monkey].items.push_back(value);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    // Return the multiplication of the *two* highest values
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    if let [a, b] = &inspections[..2] {
        Ok(a * b)
    } else {
        Err(error!("Could not find two highest values"))
    }
}

fn part2(input: &str) -> Result<usize> {
    let rounds = 10_000;
    let mut monkeys = parse::parse_monkeys(input)?;

    let common_div = monkeys.iter().map(|m| m.test).product::<usize>();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            // Go through each item and process them
            while let Some(value) = monkeys[i].items.pop_front() {
                let value = monkeys[i].apply_operation(value);
                let next_monkey = monkeys[i].get_next_monkey(value);
                monkeys[next_monkey].items.push_back(value % common_div);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    // Return the multiplication of the *two* highest values
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    if let [a, b] = &inspections[..2] {
        Ok(a * b)
    } else {
        Err(error!("Could not find two highest values"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 10_605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 2_713_310_158);
    }

    #[test]
    fn test_monkey_apply_operation() {
        let mut monkey = Monkey {
            num: 0,
            items: vec![1, 2, 3].into(),
            operation: Operation::Add(OperationValue::Number(2)),
            test: 2,
            if_true: 1,
            if_false: 2,
            inspections: 0,
        };
        assert_eq!(monkey.apply_operation(2), 4);
        assert_eq!(monkey.inspections, 1);

        let mut monkey = Monkey {
            num: 0,
            items: vec![1, 2, 3].into(),
            operation: Operation::Multiply(OperationValue::Old),
            test: 2,
            if_true: 1,
            if_false: 2,
            inspections: 0,
        };
        assert_eq!(monkey.apply_operation(3), 9);
    }

    #[test]
    fn test_monkey_get_next_monkey() {
        let monkey = Monkey {
            num: 0,
            items: vec![1, 2, 3].into(),
            operation: Operation::Add(OperationValue::Number(2)),
            test: 2,
            if_true: 1,
            if_false: 2,
            inspections: 0,
        };
        assert_eq!(monkey.get_next_monkey(4), 1);
        assert_eq!(monkey.get_next_monkey(5), 2);
    }

    #[test]
    fn test_monkey_reduce_worry() {
        let monkey = Monkey {
            num: 0,
            items: vec![1, 2, 3].into(),
            operation: Operation::Add(OperationValue::Number(2)),
            test: 2,
            if_true: 1,
            if_false: 2,
            inspections: 0,
        };

        assert_eq!(monkey.reduce_worry(12), 4);
        assert_eq!(monkey.reduce_worry(11), 3);
        assert_eq!(monkey.reduce_worry(10), 3);
        assert_eq!(monkey.reduce_worry(9), 3);
        assert_eq!(monkey.reduce_worry(8), 2);
    }
}
