#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
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
