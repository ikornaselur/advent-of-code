#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum OrdinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    pub fn opposite(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::West,
        }
    }
}

impl OrdinalDirection {
    pub fn opposite(&self) -> Self {
        match self {
            OrdinalDirection::Up => OrdinalDirection::Down,
            OrdinalDirection::Down => OrdinalDirection::Up,
            OrdinalDirection::Left => OrdinalDirection::Right,
            OrdinalDirection::Right => OrdinalDirection::Left,
        }
    }

    /// From a single character, U/D/L/R
    pub fn from_UDLR(c: &str) -> Option<Self> {
        match c {
            "U" => Some(OrdinalDirection::Up),
            "D" => Some(OrdinalDirection::Down),
            "L" => Some(OrdinalDirection::Left),
            "R" => Some(OrdinalDirection::Right),
            _ => None,
        }
    }

    /// From a single symbol, ^/v/</>
    pub fn from_symbol(s: &str) -> Option<Self> {
        match s {
            "^" => Some(OrdinalDirection::Up),
            "v" => Some(OrdinalDirection::Down),
            "<" => Some(OrdinalDirection::Left),
            ">" => Some(OrdinalDirection::Right),
            _ => None,
        }
    }
}
