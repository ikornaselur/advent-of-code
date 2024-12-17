use advent::prelude::*;

struct Schematic {
    rows: Vec<String>,
    width: usize,
    height: usize,
    gear_map: HashMap<(usize, usize), Vec<u32>>,
}

enum SymbolType {
    Gear,
    Other, // We don't really care what the other symbols are
}

trait Symbol {
    fn get_symbol_type(&self) -> Option<SymbolType>;
    fn is_symbol(&self) -> bool {
        self.get_symbol_type().is_some()
    }
}

impl Symbol for char {
    fn get_symbol_type(&self) -> Option<SymbolType> {
        match self {
            '*' => Some(SymbolType::Gear),
            '.' | '0'..='9' => None,
            _ => Some(SymbolType::Other),
        }
    }
}

impl FromStr for Schematic {
    type Err = AdventError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let height = rows.len();
        let width = rows[0].len();
        Ok(Self {
            rows,
            width,
            height,
            gear_map: HashMap::new(),
        })
    }
}

impl Schematic {
    /// Returns a list of all the part numbers in the schematic.
    ///
    /// A number in the schematic is considered a part number only if it is adjacent to any symbol
    fn get_part_numbers(&self) -> Result<Vec<u32>> {
        let mut part_numbers = Vec::new();

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut number = 0; // We can treat 0 as not a number, because we need to return the
            let mut is_part_number = false;

            for (col_index, col) in row.chars().enumerate() {
                if col.is_ascii_digit() {
                    number =
                        number * 10 + col.to_digit(10).ok_or(AdventError::InvalidDigit(col))?;
                    // Check adjacent cells to see if there are symbols
                    if self.is_adjacent_to_symbol(row_index, col_index)? {
                        is_part_number = true;
                    }
                }
                if (!col.is_ascii_digit() || col_index == self.width - 1) && number > 0 {
                    if is_part_number {
                        part_numbers.push(number);
                    }
                    number = 0;
                    is_part_number = false;
                }
            }
        }

        Ok(part_numbers)
    }

    /// Build gear map
    ///
    /// By going through all the numbers in the schematic, we can associate them to any adjacent
    /// gear by adding the number to the gear map.
    fn build_gear_map(&mut self) -> Result<()> {
        for (row_index, row) in self.rows.iter().enumerate() {
            let mut number = 0; // We can treat 0 as not a number, because we need to return the
            let mut adjacent_gear_coords = HashSet::new();

            for (col_index, col) in row.chars().enumerate() {
                if col.is_ascii_digit() {
                    number =
                        number * 10 + col.to_digit(10).ok_or(AdventError::InvalidDigit(col))?;
                    // Check adjacent cells to see if there are symbols
                    for direction in [
                        GridDirection::Up,
                        GridDirection::Down,
                        GridDirection::Right,
                        GridDirection::Left,
                        GridDirection::UpLeft,
                        GridDirection::UpRight,
                        GridDirection::DownLeft,
                        GridDirection::DownRight,
                    ] {
                        let (x, y) = match self.shift_coordinate(direction, row_index, col_index) {
                            Some((x, y)) => (x, y),
                            None => continue,
                        };
                        if let Some(SymbolType::Gear) = self.get_char(x, y)?.get_symbol_type() {
                            adjacent_gear_coords.insert((x, y));
                        }
                    }
                }
                if (!col.is_ascii_digit() || col_index == self.width - 1) && number > 0 {
                    for coord in &adjacent_gear_coords {
                        let gear_numbers = self.gear_map.entry(*coord).or_default();
                        gear_numbers.push(number);
                    }
                    number = 0;
                    adjacent_gear_coords.clear();
                }
            }
        }

        Ok(())
    }

    fn shift_coordinate(
        &self,
        direction: GridDirection,
        row_index: usize,
        col_index: usize,
    ) -> Option<(usize, usize)> {
        // Guard against out of bounds
        match direction {
            GridDirection::Up | GridDirection::UpLeft | GridDirection::UpRight
                if row_index == 0 =>
            {
                return None;
            }
            GridDirection::Down | GridDirection::DownLeft | GridDirection::DownRight
                if row_index >= self.height - 1 =>
            {
                return None;
            }
            GridDirection::Left | GridDirection::UpLeft | GridDirection::DownLeft
                if col_index == 0 =>
            {
                return None;
            }
            GridDirection::Right | GridDirection::UpRight | GridDirection::DownRight
                if col_index >= self.width - 1 =>
            {
                return None;
            }
            _ => {}
        }

        Some(match direction {
            GridDirection::Up => (row_index - 1, col_index),
            GridDirection::Down => (row_index + 1, col_index),
            GridDirection::Right => (row_index, col_index + 1),
            GridDirection::Left => (row_index, col_index - 1),
            GridDirection::UpLeft => (row_index - 1, col_index - 1),
            GridDirection::UpRight => (row_index - 1, col_index + 1),
            GridDirection::DownLeft => (row_index + 1, col_index - 1),
            GridDirection::DownRight => (row_index + 1, col_index + 1),
        })
    }

    /// Return the charater, if any, adjacent to a coordinate
    fn get_char(&self, x: usize, y: usize) -> Result<char> {
        self.rows[x].chars().nth(y).ok_or(invalid_coordinate!(x, y))
    }

    /// Return if there is a symbol at adjacent cells
    fn is_adjacent_to_symbol(&self, row_index: usize, col_index: usize) -> Result<bool> {
        for direction in [
            GridDirection::Up,
            GridDirection::Down,
            GridDirection::Right,
            GridDirection::Left,
            GridDirection::UpLeft,
            GridDirection::UpRight,
            GridDirection::DownLeft,
            GridDirection::DownRight,
        ] {
            let (x, y) = match self.shift_coordinate(direction, row_index, col_index) {
                Some((x, y)) => (x, y),
                None => continue,
            };
            if self.get_char(x, y)?.is_symbol() {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

fn main() -> Result<()> {
    let input = get_input(2023, 3)?;

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

fn part1(input: &str) -> Result<u32> {
    let schematic: Schematic = input.parse()?;

    Ok(schematic.get_part_numbers()?.iter().sum())
}

fn part2(input: &str) -> Result<u32> {
    let mut schematic: Schematic = input.parse()?;

    schematic.build_gear_map()?;

    // Find all gears that have two numbers adjacent
    let mut ratios_sum = 0;

    for (_, numbers) in schematic.gear_map.iter() {
        if numbers.len() == 2 {
            ratios_sum += numbers[0] * numbers[1];
        }
    }

    Ok(ratios_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 467835);
    }

    #[test]
    fn test_schematic_from_str() {
        let schematic: Schematic = ".#.\n123\n$*#".parse().unwrap();

        assert_eq!(schematic.rows, vec![".#.", "123", "$*#"]);
    }

    #[test]
    fn test_shift_coordinate() {
        let schematic: Schematic = ".#.\n123\n$*#".parse().unwrap();

        assert_eq!(schematic.shift_coordinate(GridDirection::Up, 0, 1), None);
        assert_eq!(
            schematic.shift_coordinate(GridDirection::Up, 1, 1),
            Some((0, 1))
        );
        assert_eq!(
            schematic.shift_coordinate(GridDirection::Down, 1, 1),
            Some((2, 1))
        );
        assert_eq!(
            schematic.shift_coordinate(GridDirection::Right, 1, 1),
            Some((1, 2))
        );
        assert_eq!(
            schematic.shift_coordinate(GridDirection::Left, 1, 1),
            Some((1, 0))
        );
        assert_eq!(schematic.shift_coordinate(GridDirection::Right, 2, 2), None);
        assert_eq!(schematic.shift_coordinate(GridDirection::Down, 2, 2), None);
        assert_eq!(
            schematic.shift_coordinate(GridDirection::UpLeft, 1, 1),
            Some((0, 0))
        );
        assert_eq!(
            schematic.shift_coordinate(GridDirection::DownLeft, 1, 1),
            Some((2, 0))
        );
        assert_eq!(
            schematic.shift_coordinate(GridDirection::DownRight, 1, 1),
            Some((2, 2,))
        );
    }

    #[test]
    fn test_get_char() {
        let schematic: Schematic = ".#.\n123\n$*#".parse().unwrap();

        assert_eq!(schematic.get_char(0, 1).unwrap(), '#');
        assert_eq!(schematic.get_char(2, 1).unwrap(), '*');
        assert_eq!(schematic.get_char(1, 2).unwrap(), '3');
        assert_eq!(schematic.get_char(1, 0).unwrap(), '1');
        assert_eq!(schematic.get_char(0, 0).unwrap(), '.');
        assert_eq!(schematic.get_char(2, 0).unwrap(), '$');
        assert_eq!(schematic.get_char(2, 2).unwrap(), '#');
    }

    #[test]
    fn test_is_symbol() {
        assert!('#'.is_symbol());
        assert!('*'.is_symbol());
        assert!('$'.is_symbol());
        assert!(!'.'.is_symbol());
        assert!(!'7'.is_symbol());
    }

    #[test]
    fn test_is_adjacent_to_symbol() {
        let schematic: Schematic = "...\n123\n..#".parse().unwrap();

        assert!(!schematic.is_adjacent_to_symbol(1, 0).unwrap());
        assert!(schematic.is_adjacent_to_symbol(1, 1).unwrap());
        assert!(schematic.is_adjacent_to_symbol(1, 2).unwrap());
    }

    #[test]
    fn test_get_part_numbers() {
        let schematic: Schematic = ".#.\n123\n..#\n...\n456".parse().unwrap();
        assert_eq!(schematic.get_part_numbers().unwrap(), vec![123]);
    }

    #[test]
    fn test_build_gear_map() {
        let mut schematic: Schematic = ".*.\n123\n..#\n...\n456".parse().unwrap();

        schematic.build_gear_map().unwrap();

        assert_eq!(schematic.gear_map.get(&(0, 1)).unwrap(), &vec![123]);
    }
}
