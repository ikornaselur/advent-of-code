use crate::{Monkey, Operation, OperationValue};
use advent::prelude::*;

/// Parse the input header in the form:
/// `Monkey <usize>`
fn nom_monkey_header(input: &str) -> IResult<&str, usize> {
    delimited(
        tag("Monkey "),
        map_res(digit1, |s: &str| s.parse::<usize>()),
        tag(":"),
    )(input)
}

/// Parse the starting items in the form:
/// `Starting items: <usize>, <usize>
///
/// Note that the number of starting items is unknown and could be just one
fn nom_starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), map_res(digit1, |s: &str| s.parse::<usize>())),
    )(input)
}

/// Helper parser for `nom_operation` below
fn nom_op_value(input: &str) -> IResult<&str, OperationValue> {
    alt((
        map_res(digit1, |s: &str| {
            s.parse::<usize>().map(OperationValue::Number)
        }),
        value(OperationValue::Old, tag("old")),
    ))(input)
}

/// Parse the operation in the form:
/// `Operation: new = old <operator> <value>
/// Where the operator is either `*` or `+` and the value can be usize or the string 'old'
/// For example:
///     * `Operation: new = old + 3`
///     * `Operation: new = old * old`
fn nom_operation(input: &str) -> IResult<&str, Operation> {
    preceded(
        tag("Operation: new = old "),
        alt((
            map(preceded(tag("+ "), nom_op_value), Operation::Add),
            map(preceded(tag("* "), nom_op_value), Operation::Multiply),
        )),
    )(input)
}

/// Parse the test in the form:
/// `Test: divisible by <usize>`
fn nom_test(input: &str) -> IResult<&str, usize> {
    map_res(preceded(tag("Test: divisible by "), digit1), |s: &str| {
        s.parse::<usize>()
    })(input)
}

/// Parse the 'if true' branch in the form:
/// `If true: throw to monkey <usize>`
fn nom_if_true(input: &str) -> IResult<&str, usize> {
    map_res(
        preceded(tag("If true: throw to monkey "), digit1),
        |s: &str| s.parse::<usize>(),
    )(input)
}

/// Parse the 'if false' branch in the form:
/// `If false: throw to monkey <usize>`
fn nom_if_false(input: &str) -> IResult<&str, usize> {
    map_res(
        preceded(tag("If false: throw to monkey "), digit1),
        |s: &str| s.parse::<usize>(),
    )(input)
}

/// Parse a full input of an individual monkey in the form:
/// ```
/// Monkey <usize>
///   Starting items: <usize>, <usize>
///   Operation: new = old <operator> <value>
///   Test: divisible by <usize>
///     If true: throw to monkey <usize>
///     If false: throw to monkey <usize>
/// ```
/// utilising the parsers defined above. Make note of the spaces before all lines, except the
/// header
fn nom_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, monkey_num) = delimited(space0, nom_monkey_header, newline)(input)?;
    let (input, starting_items) = delimited(space0, nom_starting_items, newline)(input)?;
    let (input, operation) = delimited(space0, nom_operation, newline)(input)?;
    let (input, test) = delimited(space0, nom_test, newline)(input)?;
    let (input, if_true) = delimited(space0, nom_if_true, newline)(input)?;
    let (input, if_false) = preceded(space0, nom_if_false)(input)?;

    let monkey = Monkey {
        num: monkey_num,
        items: starting_items.into(),
        operation,
        test,
        if_true,
        if_false,
        inspections: 0,
    };

    Ok((input, monkey))
}

/// Parse all the monkeys in the input
///
/// Note that each 'monkey' input is separated by an empty line
pub fn parse_monkeys(input: &str) -> Result<Vec<Monkey>> {
    let mut parser = separated_list1(tag("\n\n"), nom_monkey);
    let (_, monkeys) = parser(input).map_err(|e| error!("Unable to parse: {}", e))?;

    Ok(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nom_monkey_header() {
        let (rest, result) = nom_monkey_header("Monkey 42:").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, 42);
    }

    #[test]
    fn test_nom_starting_items() {
        let (rest, result) = nom_starting_items("Starting items: 3, 4, 5").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, vec![3, 4, 5]);

        let (rest, result) = nom_starting_items("Starting items: 3").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_nom_op_value() {
        let (rest, result) = nom_op_value("3").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, OperationValue::Number(3));

        let (rest, result) = nom_op_value("old").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, OperationValue::Old);
    }

    #[test]
    fn test_nom_operation() {
        let (rest, result) = nom_operation("Operation: new = old + 3").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, Operation::Add(OperationValue::Number(3)));

        let (rest, result) = nom_operation("Operation: new = old + old").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, Operation::Add(OperationValue::Old));

        let (rest, result) = nom_operation("Operation: new = old * 3").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, Operation::Multiply(OperationValue::Number(3)));
    }

    #[test]
    fn test_nom_test() {
        let (rest, result) = nom_test("Test: divisible by 17").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, 17);
    }

    #[test]
    fn test_nom_if_true() {
        let (rest, result) = nom_if_true("If true: throw to monkey 42").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, 42);
    }

    #[test]
    fn test_nom_if_false() {
        let (rest, result) = nom_if_false("If false: throw to monkey 38").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, 38);
    }

    #[test]
    fn test_nom_monkey() {
        let input = [
            "Monkey 1:",
            "  Starting items: 1, 2, 3",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 42",
            "    If false: throw to monkey 38",
        ]
        .join("\n");

        let (rest, result) = nom_monkey(&input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Monkey {
                num: 1,
                items: vec![1, 2, 3].into(),
                operation: Operation::Add(OperationValue::Number(3)),
                test: 17,
                if_true: 42,
                if_false: 38,
                inspections: 0,
            }
        )
    }

    #[test]
    fn test_parse_monkeys() {
        let input = [
            "Monkey 1:",
            "  Starting items: 1, 2, 3",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 42",
            "    If false: throw to monkey 38",
            "",
            "Monkey 2:",
            "  Starting items: 4, 5, 6",
            "  Operation: new = old * old",
            "  Test: divisible by 3",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 2",
        ]
        .join("\n");

        let monkeys = parse_monkeys(&input).unwrap();

        assert_eq!(monkeys.len(), 2);
    }
}
