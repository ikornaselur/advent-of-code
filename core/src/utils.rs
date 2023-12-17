use crate::types::Coordinate;

/// Calculate the manhattan distance between two coordinates
pub fn manhattan_distance(from: Coordinate, to: Coordinate) -> usize {
    let x = if from.0 > to.0 {
        from.0 - to.0
    } else {
        to.0 - from.0
    };
    let y = if from.1 > to.1 {
        from.1 - to.1
    } else {
        to.1 - from.1
    };

    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0), (0, 0)), 0);
        assert_eq!(manhattan_distance((0, 0), (1, 0)), 1);
        assert_eq!(manhattan_distance((0, 0), (0, 1)), 1);
        assert_eq!(manhattan_distance((1, 0), (0, 0)), 1);
        assert_eq!(manhattan_distance((0, 1), (0, 0)), 1);

        assert_eq!(manhattan_distance((12, 71), (125, 51)), 133);
    }
}
