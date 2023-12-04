#[derive(Debug)]
pub enum AdventError {
    ConversionError,
    InvalidInput,
    InvalidDigit(char),
    NoNumbers,
    ParseError(String),
    InvalidCoordinate { row: usize, col: usize },
}

impl std::fmt::Display for AdventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdventError::ConversionError => write!(f, "Unable to convert character to digit"),
            AdventError::InvalidInput => write!(f, "Invalid input"),
            AdventError::InvalidDigit(c) => write!(f, "Invalid digit: {}", c),
            AdventError::NoNumbers => write!(f, "No numbers found in the input"),
            AdventError::ParseError(msg) => write!(f, "Unable to parse: {}", msg),
            AdventError::InvalidCoordinate { row, col } => {
                write!(f, "Invalid coordinate: ({}, {})", row, col)
            }
        }
    }
}

impl std::error::Error for AdventError {}

impl From<std::num::ParseIntError> for AdventError {
    fn from(error: std::num::ParseIntError) -> Self {
        AdventError::ParseError(error.to_string())
    }
}

#[macro_export]
macro_rules! invalid_coordinate {
    ($x:expr, $y:expr) => {
        AdventError::InvalidCoordinate { row: $x, col: $y }
    };
}

#[macro_export]
macro_rules! parse_error {
    ($e:expr) => {
        AdventError::ParseError($e.to_string())
    };
}
