use nom::error::Error as NomError;
use nom::Err as NomErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Error: {0}")]
    Error(String),
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
    #[error("Unable to parse: {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Invalid coordinate: ({row}, {col})")]
    InvalidCoordinate { row: usize, col: usize },
    #[error("Parsing error: {0}")]
    NomError(String),
}

impl<I: std::fmt::Debug> From<NomErr<NomError<I>>> for AdventError {
    fn from(err: NomErr<NomError<I>>) -> Self {
        match err {
            NomErr::Incomplete(_) => AdventError::NomError("Incomplete input".to_string()),
            NomErr::Error(e) => AdventError::NomError(format!("{:?}", e)),
            NomErr::Failure(e) => AdventError::NomError(format!("{:?}", e)),
        }
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

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => {
        AdventError::Error(format!($($t)*))
    };
}
