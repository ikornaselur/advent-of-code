#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum CompassDirection {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    West,
    East,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum GridDirection {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
}

impl CompassDirection {
    pub fn opposite(&self) -> Self {
        match self {
            CompassDirection::North => CompassDirection::South,
            CompassDirection::South => CompassDirection::North,
            CompassDirection::West => CompassDirection::East,
            CompassDirection::East => CompassDirection::West,
            CompassDirection::NorthWest => CompassDirection::SouthEast,
            CompassDirection::SouthEast => CompassDirection::NorthWest,
            CompassDirection::SouthWest => CompassDirection::NorthEast,
            CompassDirection::NorthEast => CompassDirection::SouthWest,
        }
    }

    pub fn right_90(&self) -> Self {
        match self {
            CompassDirection::North => CompassDirection::East,
            CompassDirection::East => CompassDirection::South,
            CompassDirection::South => CompassDirection::West,
            CompassDirection::West => CompassDirection::North,
            CompassDirection::NorthWest => CompassDirection::NorthEast,
            CompassDirection::NorthEast => CompassDirection::SouthEast,
            CompassDirection::SouthEast => CompassDirection::SouthWest,
            CompassDirection::SouthWest => CompassDirection::NorthWest,
        }
    }

    pub fn left_90(&self) -> Self {
        match self {
            CompassDirection::North => CompassDirection::West,
            CompassDirection::West => CompassDirection::South,
            CompassDirection::South => CompassDirection::East,
            CompassDirection::East => CompassDirection::North,
            CompassDirection::NorthWest => CompassDirection::SouthWest,
            CompassDirection::SouthWest => CompassDirection::SouthEast,
            CompassDirection::SouthEast => CompassDirection::NorthEast,
            CompassDirection::NorthEast => CompassDirection::NorthWest,
        }
    }

    pub fn as_vector(&self) -> (i32, i32) {
        // TODO: This doesn't match as_vector in GridDirection.. so need to make sure it's
        // refactored to match!
        // This is the right way, if we think of it as (row, column) and we start in upper-right
        // corner
        match self {
            CompassDirection::North => (-1, 0),
            CompassDirection::South => (1, 0),
            CompassDirection::West => (0, -1),
            CompassDirection::East => (0, 1),
            CompassDirection::NorthWest => (-1, -1),
            CompassDirection::NorthEast => (-1, 1),
            CompassDirection::SouthWest => (1, -1),
            CompassDirection::SouthEast => (1, 1),
        }
    }
}

impl GridDirection {
    pub fn opposite(&self) -> Self {
        match self {
            GridDirection::Up => GridDirection::Down,
            GridDirection::Down => GridDirection::Up,
            GridDirection::Left => GridDirection::Right,
            GridDirection::Right => GridDirection::Left,
            GridDirection::UpLeft => GridDirection::DownRight,
            GridDirection::DownRight => GridDirection::UpLeft,
            GridDirection::DownLeft => GridDirection::UpRight,
            GridDirection::UpRight => GridDirection::DownLeft,
        }
    }

    /// Turn the direction into a "unit vector"
    ///
    /// If we assume the grid system is (y, x) and the grid grows down and to the right, then "up"
    /// is (0, -1) and "left" is (-1, 0) for example.
    ///
    /// Why (y, x)? Because when working with a grid of Vec<Vec<_>> the first value will be the
    /// height (y) and the second value is the width (x) and it's simpler to think of it as
    /// grid[y][x] to access a cell.
    pub fn as_vector(&self) -> (i32, i32) {
        match self {
            GridDirection::Up => (0, -1),
            GridDirection::Down => (0, 1),
            GridDirection::Left => (-1, 0),
            GridDirection::Right => (1, 0),
            GridDirection::UpLeft => (-1, -1),
            GridDirection::DownRight => (1, 1),
            GridDirection::DownLeft => (-1, 1),
            GridDirection::UpRight => (1, -1),
        }
    }

    pub fn directions() -> Vec<Self> {
        vec![
            GridDirection::Up,
            GridDirection::Down,
            GridDirection::Left,
            GridDirection::Right,
            GridDirection::UpLeft,
            GridDirection::DownRight,
            GridDirection::DownLeft,
            GridDirection::UpRight,
        ]
    }

    /// From a single character, U/D/L/R
    pub fn from_udlr(c: &str) -> Option<Self> {
        match c {
            "U" => Some(GridDirection::Up),
            "D" => Some(GridDirection::Down),
            "L" => Some(GridDirection::Left),
            "R" => Some(GridDirection::Right),
            _ => None,
        }
    }

    /// From a single symbol, ^/v/</>
    pub fn from_symbol(s: &str) -> Option<Self> {
        match s {
            "^" => Some(GridDirection::Up),
            "v" => Some(GridDirection::Down),
            "<" => Some(GridDirection::Left),
            ">" => Some(GridDirection::Right),
            _ => None,
        }
    }
}
