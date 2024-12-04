use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");
const XMAS: &[char; 4] = &['X', 'M', 'A', 'S'];

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn search_xmas(grid: &[&[char]], y: usize, x: usize, direction: GridDirection) -> bool {
    let vector = direction.as_vector();

    match direction {
        GridDirection::Up | GridDirection::UpLeft | GridDirection::UpRight if y < 3 => {
            return false;
        }
        GridDirection::Down | GridDirection::DownLeft | GridDirection::DownRight
            if y + 4 > grid.len() =>
        {
            return false;
        }
        GridDirection::Left | GridDirection::DownLeft | GridDirection::UpLeft if x < 3 => {
            return false;
        }
        GridDirection::Right | GridDirection::DownRight | GridDirection::UpRight
            if x + 4 > grid[0].len() =>
        {
            return false;
        }
        _ => {}
    };

    for (i, c) in XMAS.iter().enumerate() {
        let y = ((y as i32) + (i as i32) * vector.1) as usize;
        let x = ((x as i32) + (i as i32) * vector.0) as usize;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> Result<usize> {
    let grid = parse_input(input)?;
    let grid_slices: Vec<&[char]> = grid.iter().map(|v| v.as_slice()).collect();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut xmas_count = 0;

    let directions = GridDirection::directions();

    for y in 0..grid_height {
        for x in 0..grid_width {
            xmas_count += directions
                .iter()
                .filter(|direction| search_xmas(&grid_slices, y, x, **direction))
                .count()
        }
    }

    Ok(xmas_count)
}

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_search_xmas() {
        let grid = [vec!['X', 'M', 'A', 'S'], vec!['S', 'A', 'M', 'X']];
        let slices: Vec<&[char]> = grid.iter().map(|v| v.as_slice()).collect();

        assert!(search_xmas(&slices, 0, 0, GridDirection::Right));
        assert!(search_xmas(&slices, 1, 3, GridDirection::Left));
        assert!(!search_xmas(&slices, 1, 0, GridDirection::Left));
        assert!(!search_xmas(&slices, 0, 3, GridDirection::Right));
    }
}
