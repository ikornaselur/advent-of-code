pub use crate::enums::{CardinalDirection, OrdinalDirection};
pub use crate::error_handling::AdventError;
pub use crate::types::Coordinate;
pub use crate::utils::manhattan_distance;
pub use crate::{error, invalid_coordinate, parse_error};

// Also include the common imports for all the days
pub use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
pub use std::str::FromStr;
pub type Result<T> = std::result::Result<T, AdventError>;
