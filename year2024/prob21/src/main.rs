use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fmt;
use std::fs;

mod parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Button {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
    Left,
    Right,
    Up,
    Down,
}

fn opposite_direction(button: &Button) -> Button {
    match button {
        Button::Left => Button::Right,
        Button::Right => Button::Left,
        Button::Up => Button::Down,
        Button::Down => Button::Up,
        _ => panic!("Not a direction"),
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Button::One => '1',
            Button::Two => '2',
            Button::Three => '3',
            Button::Four => '4',
            Button::Five => '5',
            Button::Six => '6',
            Button::Seven => '7',
            Button::Eight => '8',
            Button::Nine => '9',
            Button::Zero => '0',
            Button::A => 'A',
            Button::Left => '<',
            Button::Right => '>',
            Button::Up => '^',
            Button::Down => 'v',
        };
        write!(f, "{}", c)
    }
}

struct Pad {
    // A hashmap containing which buttons are 'connected'
    // Each connection is indicated as a tuple of (<Connected button>, <Direction>)
    buttons: HashMap<Button, Vec<(Button, Button)>>,
}

impl Pad {
    fn new_number_pad() -> Self {
        let buttons = HashMap::from([
            (
                Button::One,
                vec![(Button::Two, Button::Right), (Button::Four, Button::Up)],
            ),
            (
                Button::Two,
                vec![
                    (Button::One, Button::Left),
                    (Button::Five, Button::Up),
                    (Button::Three, Button::Right),
                    (Button::Zero, Button::Down),
                ],
            ),
            (
                Button::Three,
                vec![
                    (Button::Two, Button::Left),
                    (Button::Six, Button::Up),
                    (Button::A, Button::Down),
                ],
            ),
            (
                Button::Four,
                vec![
                    (Button::One, Button::Down),
                    (Button::Five, Button::Right),
                    (Button::Seven, Button::Up),
                ],
            ),
            (
                Button::Five,
                vec![
                    (Button::Two, Button::Down),
                    (Button::Four, Button::Left),
                    (Button::Six, Button::Right),
                    (Button::Eight, Button::Up),
                ],
            ),
            (
                Button::Six,
                vec![
                    (Button::Three, Button::Down),
                    (Button::Five, Button::Left),
                    (Button::Nine, Button::Up),
                ],
            ),
            (
                Button::Seven,
                vec![(Button::Four, Button::Down), (Button::Eight, Button::Right)],
            ),
            (
                Button::Eight,
                vec![
                    (Button::Five, Button::Down),
                    (Button::Seven, Button::Left),
                    (Button::Nine, Button::Right),
                ],
            ),
            (
                Button::Nine,
                vec![(Button::Six, Button::Down), (Button::Eight, Button::Left)],
            ),
            (
                Button::Zero,
                vec![(Button::Two, Button::Up), (Button::A, Button::Right)],
            ),
            (
                Button::A,
                vec![(Button::Three, Button::Up), (Button::Zero, Button::Left)],
            ),
        ]);

        Pad { buttons }
    }

    fn new_direction_pad() -> Self {
        let buttons = HashMap::from([
            (Button::Left, vec![(Button::Down, Button::Right)]),
            (
                Button::Right,
                vec![(Button::Down, Button::Left), (Button::A, Button::Up)],
            ),
            (
                Button::Up,
                vec![(Button::Down, Button::Down), (Button::A, Button::Right)],
            ),
            (
                Button::Down,
                vec![
                    (Button::Left, Button::Left),
                    (Button::Right, Button::Right),
                    (Button::Up, Button::Up),
                ],
            ),
            (
                Button::A,
                vec![(Button::Right, Button::Down), (Button::Up, Button::Left)],
            ),
        ]);

        Pad { buttons }
    }
}

/// Check how many moves are required to move between two buttons
///
/// The two buttons given are going to be numpad buttons, which are the buttons we move between on
/// level 0. All moves on any other level is going to be on a directional pad
///
/// We'll start without any caching, as that's going to be fine for part 1.. but part 2 might be a
/// different story and we might need to figure out how we can cache moves on higher levels, which
/// should in theory be trivial?
///
/// We'll be returning a list of all paths that can be taken, though we can rule out immediately
/// some paths (such as >v>, which will always be worse than >>v or v>>
fn check_moves(code: &[Button], level: usize) -> Vec<Vec<Button>> {
    let pad = if level == 0 {
        Pad::new_number_pad()
    } else {
        Pad::new_direction_pad()
    };

    // We start from 'A', as that's the default start position
    let from = Button::A;
    let to = code[0];
    let mut paths = get_paths_from_to(&pad, from, to);

    if paths.is_empty() {
        panic!();
    }

    for idx in 0..code.len() - 1 {
        let from = code[idx];
        let to = code[idx + 1];
        let next_paths = get_paths_from_to(&pad, from, to);

        if next_paths.is_empty() {
            panic!();
        }

        paths = paths
            .into_iter()
            .flat_map(|p| {
                next_paths.iter().map(move |next_path| {
                    let mut new_path = p.clone();
                    new_path.extend(next_path.iter());
                    new_path
                })
            })
            .collect();
    }

    paths
}

fn get_paths_from_to(pad: &Pad, from: Button, to: Button) -> Vec<Vec<Button>> {
    let mut paths = vec![];

    // Score (negative) and path so far.
    // The path does *not* include the 'from' button, but will include the 'to' button
    // Note: The path is going to be directions, not the actual buttons
    let mut queue = BinaryHeap::new();
    queue.push((0, from, None, vec![]));

    let mut best_score = i32::MIN;

    while let Some((score, current, last_direction, path)) = queue.pop() {
        // If we have a best score already, we can skip further searching
        if score < best_score {
            continue;
        }

        // Have we reached the goal?
        if current == to {
            // We need to press 'A' when we've reached to goal
            let mut new_path = path.clone();
            new_path.push(Button::A);
            paths.push(new_path);
            best_score = score;
            continue;
        }

        for (next, direction) in pad.buttons[&current].iter() {
            // Turning around is never good.. so lets skip that completely
            if let Some(prev) = last_direction {
                if *direction == opposite_direction(&prev) {
                    continue;
                }
            }

            let mut new_path = path.clone();
            new_path.push(*direction);

            let dir_change_cost = match last_direction {
                Some(prev) if prev != *direction => 2, // Penalty for changing direction
                _ => 0,
            };
            let new_score = score - 1 - dir_change_cost;

            queue.push((new_score, *next, Some(*direction), new_path));
        }
    }

    paths
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2024, 21)?
    };

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
    let codes = parse_input(input)?;

    let mut complexities = 0;
    let levels = 2;

    for code in codes {
        let mut paths = check_moves(&code, 0);

        for _ in 0..levels {
            let mut next_paths = vec![];
            for path in &paths {
                next_paths.extend(check_moves(path, 1));
            }
            paths = next_paths;
        }

        let shortest_path = paths.iter().map(|p| p.len()).min().unwrap();

        // Convert the code to a String
        let code_num: usize = code
            .iter()
            .take(code.len() - 1)
            .map(|b| b.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        complexities += code_num * shortest_path;
    }

    Ok(complexities)
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 126_384);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
