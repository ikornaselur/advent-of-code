pub use crate::benchmark::benchmark_parts;
pub use crate::enums::{CompassDirection, GridDirection};
pub use crate::error_handling::AdventError;
pub use crate::timeouts::run_with_timeout;
pub use crate::types::Coordinate;
pub use crate::utils::manhattan_distance;
pub use crate::{error, invalid_coordinate, parse_error};

// Also include the common imports for all the days
pub use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
pub use std::str::FromStr;
pub type Result<T> = std::result::Result<T, AdventError>;

// Add common nom imports that are used for most parsing
pub use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{
        alpha0, alpha1, char, digit0, digit1, line_ending, multispace0, multispace1, newline,
        not_line_ending, one_of, satisfy, space0, space1,
    },
    character::streaming::anychar,
    combinator::{map, map_res, opt, recognize, success, value, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
