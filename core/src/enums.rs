#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
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
