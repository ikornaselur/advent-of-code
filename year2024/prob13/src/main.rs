use advent::prelude::*;
use parse::parse_input;

mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Button {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Problem {
    a: Button,
    b: Button,
    prize: (u64, u64),
}

fn main() -> Result<()> {
    let input = get_input(2024, 13)?;

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

/// Best name function so far this year
///
/// Here we solve the two linear equations (I think that's what they're called?), with the
/// assumption that they overlap in one place and they only use whole numbers.
///
/// The return value is a tuple of how many times A needs to be pressed and how many times B needs
/// to be pressed (the A and B to solve the equations)
///
/// Button A: X+94, Y+34
/// Button B: X+22, Y+67
/// Prize: X=8400, Y=5400
fn solve_problem(problem: Problem) -> Option<(usize, usize)> {
    // Equation 1 is Ax + Bx = PrizeX, represented by a tuple
    // Equation 2 is Ay + By = PrizeY
    let mut eq1 = (
        problem.a.x as i64,
        problem.b.x as i64,
        problem.prize.0 as i64,
    );
    let mut eq2 = (
        problem.a.y as i64,
        problem.b.y as i64,
        problem.prize.1 as i64,
    );

    let ay = eq2.0;
    let ax = eq1.0;
    // Multiply equation 1 by Ay and equation 2 by Ax
    eq1 = (eq1.0 * ay, eq1.1 * ay, eq1.2 * ay);
    eq2 = (eq2.0 * ax, eq2.1 * ax, eq2.2 * ax);

    if eq1.0 != eq2.0 {
        panic!("Equations are not compatible");
    }
    // Now we have matching terms, so we subtract eq1 from eq2 (or swapped if eq1 is bigger)
    let eq = if eq1.1 > eq2.1 {
        (0, eq1.1 - eq2.1, eq1.2 - eq2.2)
    } else {
        (0, eq2.1 - eq1.1, eq2.2 - eq1.2)
    };

    // Solve B
    // If we can't divide without remainders, we don't have a solution
    if eq.2 % eq.1 != 0 {
        return None;
    }
    let b = eq.2 / eq.1;

    // Then insert B into original equation 1
    let eq1 = (
        problem.a.x as i64,
        problem.b.x as i64 * b,
        problem.prize.0 as i64,
    );
    // Similarly, if we can't divide without remaindes, we don't have a solution
    if (eq1.2 - eq1.1) % eq1.0 != 0 {
        return None;
    }
    let a = (eq1.2 - eq1.1) / eq1.0;

    if a < 0 || b < 0 {
        panic!("Equations are not compatible");
    }

    Some((a as usize, b as usize))
}

fn part1(input: &str) -> Result<usize> {
    let problems = parse_input(input)?;

    Ok(problems.iter().fold(0, |acc, problem| {
        if let Some((a, b)) = solve_problem(*problem) {
            acc + (a * 3) + b
        } else {
            acc
        }
    }))
}

fn part2(input: &str) -> Result<usize> {
    let problems = parse_input(input)?;
    let extra: u64 = 10_000_000_000_000;

    Ok(problems.iter().fold(0, |acc, problem| {
        if let Some((a, b)) = solve_problem(Problem {
            a: problem.a,
            b: problem.b,
            prize: (problem.prize.0 + extra, problem.prize.1 + extra),
        }) {
            acc + (a * 3) + b
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 875_318_608_908);
    }

    #[test]
    fn test_solve_problem() {
        let problem = Problem {
            a: Button { x: 94, y: 34 },
            b: Button { x: 22, y: 67 },
            prize: (8400, 5400),
        };

        let solution = solve_problem(problem);

        assert_eq!(solution, Some((80, 40)));
    }
}
