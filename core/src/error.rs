use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Error: {0}")]
    GenericError(String),
    #[error("Unable to convert character to digit")]
    ConversionError,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Invalid digit: {0}")]
    InvalidDigit(char),
    #[error("No numbers found in the input")]
    NoNumbers,
    #[error("Unable to parse: {0}")]
    ParseError(String),
    #[error("Unable to parse: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Invalid coordinate: ({row}, {col})")]
    InvalidCoordinate { row: usize, col: usize },
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

#[macro_export]
macro_rules! generic_error {
    ($e:expr) => {
        AdventError::GenericError($e.to_string())
    };
}
