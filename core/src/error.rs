#[derive(Debug)]
pub enum AdventError {
    ConversionError,
    ParseError,
    NoNumbers,
}

impl std::fmt::Display for AdventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdventError::ConversionError => write!(f, "Unable to convert character to digit"),
            AdventError::NoNumbers => write!(f, "No numbers found in the input"),
            AdventError::ParseError => write!(f, "Unable to parse"),
        }
    }
}

impl std::error::Error for AdventError {}

impl From<std::num::ParseIntError> for AdventError {
    fn from(_error: std::num::ParseIntError) -> Self {
        AdventError::ParseError
    }
}
