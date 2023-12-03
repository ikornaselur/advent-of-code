#[derive(Debug)]
pub enum AdventError {
    ConversionError,
    InvalidDigit(char),
    NoNumbers,
    ParseError,
    InvalidCoordinate { row: usize, col: usize },
}

impl std::fmt::Display for AdventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdventError::ConversionError => write!(f, "Unable to convert character to digit"),
            AdventError::InvalidDigit(c) => write!(f, "Invalid digit: {}", c),
            AdventError::NoNumbers => write!(f, "No numbers found in the input"),
            AdventError::ParseError => write!(f, "Unable to parse"),
            AdventError::InvalidCoordinate { row, col } => {
                write!(f, "Invalid coordinate: ({}, {})", row, col)
            }
        }
    }
}

impl std::error::Error for AdventError {}

impl From<std::num::ParseIntError> for AdventError {
    fn from(_error: std::num::ParseIntError) -> Self {
        AdventError::ParseError
    }
}

#[macro_export]
macro_rules! invalid_coordinate {
    ($x:expr, $y:expr) => {
        AdventError::InvalidCoordinate { row: $x, col: $y }
    };
}
