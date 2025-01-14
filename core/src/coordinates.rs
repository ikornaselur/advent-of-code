use crate::enums::DirectionShift;
use crate::prelude::*;
use num_traits::{FromPrimitive, PrimInt, Zero};
use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridCoordinate<T> {
    pub column: T,
    pub row: T,
}

impl<T> GridCoordinate<T>
where
    T: PrimInt + PartialOrd,
{
    pub fn new(row: T, column: T) -> Self {
        Self { row, column }
    }

    fn within_unsigned(&self, height: T, width: T) -> bool {
        self.row < height && self.column < width
    }

    fn within_signed(&self, height: T, width: T) -> bool {
        self.row >= Zero::zero()
            && self.column >= Zero::zero()
            && self.row < height
            && self.column < width
    }

    pub fn manhattan_distance(&self, other: &Self) -> T {
        let row_diff = if self.row >= other.row {
            self.row - other.row
        } else {
            other.row - self.row
        };
        let col_diff = if self.column >= other.column {
            self.column - other.column
        } else {
            other.column - self.column
        };
        row_diff + col_diff
    }
}

impl<T> GridCoordinate<T>
where
    T: PrimInt + PartialOrd + FromPrimitive,
{
    pub fn within_grid<U>(&self, grid: &[Vec<U>]) -> bool {
        if grid.is_empty() || grid[0].is_empty() {
            return false;
        }

        let height = T::from_usize(grid.len()).unwrap();
        let width = T::from_usize(grid[0].len()).unwrap();

        if T::min_value() < Zero::zero() {
            self.within_signed(height, width)
        } else {
            self.within_unsigned(height, width)
        }
    }

    pub fn get<'a, U>(&self, grid: &'a [Vec<U>]) -> Option<&'a U> {
        if !self.within_grid(grid) {
            return None;
        }

        // Safe to unwrap since we already checked bounds with within_grid
        let row = self.row.to_usize().unwrap();
        let col = self.column.to_usize().unwrap();

        Some(&grid[row][col])
    }

    pub fn set<U>(&self, grid: &mut [Vec<U>], value: U) -> Result<()> {
        if !self.within_grid(grid) {
            return Err(error!("Out of bounds"));
        }
        // Safe to unwrap since we already checked bounds with within_grid
        let row = self.row.to_usize().unwrap();
        let col = self.column.to_usize().unwrap();
        grid[row][col] = value;

        Ok(())
    }

    pub fn shifted<D: DirectionShift>(&self, direction: D) -> Option<Self> {
        let direction = direction.to_grid_direction();
        let is_signed = T::min_value() < Zero::zero();

        match direction {
            GridDirection::Up if is_signed || self.row > T::zero() => {
                Some(GridCoordinate::new(self.row - T::one(), self.column))
            }
            GridDirection::Down => Some(GridCoordinate::new(self.row + T::one(), self.column)),
            GridDirection::Left if is_signed || self.column > T::zero() => {
                Some(GridCoordinate::new(self.row, self.column - T::one()))
            }
            GridDirection::Right => Some(GridCoordinate::new(self.row, self.column + T::one())),
            GridDirection::UpLeft
                if is_signed || (self.column > T::zero() && self.row > T::zero()) =>
            {
                Some(GridCoordinate::new(
                    self.row - T::one(),
                    self.column - T::one(),
                ))
            }
            GridDirection::UpRight if is_signed || self.row > T::zero() => Some(
                GridCoordinate::new(self.row - T::one(), self.column + T::one()),
            ),
            GridDirection::DownLeft if is_signed || self.column > T::zero() => Some(
                GridCoordinate::new(self.row + T::one(), self.column - T::one()),
            ),
            GridDirection::DownRight => Some(GridCoordinate::new(
                self.row + T::one(),
                self.column + T::one(),
            )),
            _ => None,
        }
    }
}

impl<T: PrimInt> Add<(T, T)> for GridCoordinate<T> {
    type Output = Self;

    fn add(self, other: (T, T)) -> Self::Output {
        Self {
            row: self.row + other.0,
            column: self.column + other.1,
        }
    }
}

impl<T: PrimInt> Add<GridCoordinate<T>> for GridCoordinate<T> {
    type Output = Self;

    fn add(self, other: GridCoordinate<T>) -> Self::Output {
        Self {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl<T: PrimInt> AddAssign<(T, T)> for GridCoordinate<T> {
    fn add_assign(&mut self, other: (T, T)) {
        self.row = self.row + other.0;
        self.column = self.column + other.1;
    }
}

impl<T: PrimInt> AddAssign<GridCoordinate<T>> for GridCoordinate<T> {
    fn add_assign(&mut self, other: GridCoordinate<T>) {
        self.row = self.row + other.row;
        self.column = self.column + other.column;
    }
}

impl<T> GridCoordinate<T>
where
    T: FromPrimitive + Add<Output = T> + PrimInt,
{
    pub fn surrounding_coordinates(
        &self,
        max_distance: usize,
    ) -> impl Iterator<Item = GridCoordinate<T>> + '_ {
        let max_distance: i32 = max_distance as i32;
        let is_signed = T::min_value() < Zero::zero();
        let current_row = self.row.to_i32().unwrap();
        let current_col = self.column.to_i32().unwrap();

        (-max_distance..=max_distance).flat_map(move |x| {
            (-max_distance..=max_distance).filter_map(move |y| {
                let distance = x.abs() + y.abs();
                if distance == 0 || distance > max_distance {
                    return None;
                }

                let new_row = current_row + x;
                let new_col = current_col + y;

                // For unsigned types, skip negative coordinates
                if !is_signed && (new_row < 0 || new_col < 0) {
                    return None;
                }

                // Safe to unwrap here since we've checked for negative values
                Some(GridCoordinate::new(
                    T::from_i32(new_row).unwrap(),
                    T::from_i32(new_col).unwrap(),
                ))
            })
        })
    }

    pub fn edge_coordinates(
        &self,
        distance: usize,
    ) -> impl Iterator<Item = GridCoordinate<T>> + '_ {
        let distance: i32 = distance as i32;
        let is_signed = T::min_value() < Zero::zero();
        let current_row = self.row.to_i32().unwrap();
        let current_col = self.column.to_i32().unwrap();

        (-distance..=distance).flat_map(move |x| {
            (-distance..=distance).filter_map(move |y| {
                if distance == 0 || x.abs() + y.abs() != distance {
                    return None;
                }

                let new_row = current_row + x;
                let new_col = current_col + y;

                // For unsigned types, skip negative coordinates
                if !is_signed && (new_row < 0 || new_col < 0) {
                    return None;
                }

                // Safe to unwrap here since we've checked for negative values
                Some(GridCoordinate::new(
                    T::from_i32(new_row).unwrap(),
                    T::from_i32(new_col).unwrap(),
                ))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creation() {
        let coord_u32: GridCoordinate<u32> = GridCoordinate::new(5, 3);
        assert_eq!(coord_u32.row, 5);
        assert_eq!(coord_u32.column, 3);

        let coord_i32: GridCoordinate<i32> = GridCoordinate::new(-1, 2);
        assert_eq!(coord_i32.row, -1);
        assert_eq!(coord_i32.column, 2);
    }

    #[test]
    fn test_within_unsigned() {
        let coord: GridCoordinate<u32> = GridCoordinate::new(5, 3);

        assert!(coord.within_unsigned(10, 10));
        assert!(coord.within_unsigned(6, 4));

        assert!(!coord.within_unsigned(5, 10));
        assert!(!coord.within_unsigned(10, 3));
        assert!(!coord.within_unsigned(4, 4));
    }

    #[test]
    fn test_within_signed() {
        let valid_coord: GridCoordinate<i32> = GridCoordinate::new(5, 3);
        let negative_row: GridCoordinate<i32> = GridCoordinate::new(-1, 3);
        let negative_col: GridCoordinate<i32> = GridCoordinate::new(5, -1);

        assert!(valid_coord.within_signed(10, 10));
        assert!(valid_coord.within_signed(6, 4));

        assert!(!negative_row.within_signed(10, 10));
        assert!(!negative_col.within_signed(10, 10));

        assert!(!valid_coord.within_signed(5, 10));
        assert!(!valid_coord.within_signed(10, 3));
    }

    #[test]
    fn test_within_grid() {
        let grid: Vec<Vec<char>> = vec![vec!['a'; 5]; 4]; // 4x5 grid
        let empty_grid: Vec<Vec<char>> = Vec::new();
        let empty_row_grid: Vec<Vec<char>> = vec![Vec::new()];

        // Test unsigned coordinates
        let valid_u32: GridCoordinate<u32> = GridCoordinate::new(2, 3);
        let invalid_u32: GridCoordinate<u32> = GridCoordinate::new(5, 3);

        assert!(valid_u32.within_grid(&grid));
        assert!(!invalid_u32.within_grid(&grid));

        // Test signed coordinates
        let valid_i32: GridCoordinate<i32> = GridCoordinate::new(2, 3);
        let negative_i32: GridCoordinate<i32> = GridCoordinate::new(-1, 3);

        assert!(valid_i32.within_grid(&grid));
        assert!(!negative_i32.within_grid(&grid));

        // Test empty grids
        assert!(!valid_u32.within_grid(&empty_grid));
        assert!(!valid_u32.within_grid(&empty_row_grid));
    }

    #[test]
    fn test_basic_tuple_addition() {
        let coord_i32 = GridCoordinate::new(1, 2);
        let coord_u32 = GridCoordinate::new(5_u32, 7_u32);

        assert_eq!(coord_i32 + (2, 3), GridCoordinate::new(3, 5));
        assert_eq!(coord_u32 + (2, 3), GridCoordinate::new(7, 10));
    }

    #[test]
    fn test_zero_tuple_addition() {
        let coord_i32 = GridCoordinate::new(1, 2);
        let coord_u32 = GridCoordinate::new(5_u32, 7_u32);

        assert_eq!(coord_i32 + (0, 0), coord_i32);
        assert_eq!(coord_u32 + (0, 0), coord_u32);
    }

    #[test]
    fn test_negative_tuple_addition() {
        let coord = GridCoordinate::new(1, 2);

        assert_eq!(coord + (-1, -2), GridCoordinate::new(0, 0));
        assert_eq!(coord + (-5, -7), GridCoordinate::new(-4, -5));
    }

    #[test]
    fn test_large_tuple_addition_signed() {
        let coord = GridCoordinate::new(1_i32, 2_i32);
        let large_value = i32::MAX - 2;

        let expected = GridCoordinate::new(
            1 + large_value,       // 1 + (MAX-2) = MAX-1
            2 + (large_value - 1), // 2 + (MAX-3) = MAX-1
        );

        assert_eq!(coord + (large_value, large_value - 1), expected);
    }

    #[test]
    fn test_large_tuple_addition_unsigned() {
        let coord = GridCoordinate::new(5_u32, 7_u32);
        let large_value = u32::MAX - 10;

        let expected = GridCoordinate::new(
            5 + 2,                 // 7
            7 + (large_value - 7), // 7 + (MAX-17) = MAX-10
        );

        assert_eq!(coord + (2, large_value - 7), expected);
    }

    #[test]
    #[should_panic]
    fn test_tuple_addition_overflow_unsigned() {
        let coord = GridCoordinate::new(1_u32, 2_u32);
        let _ = coord + (u32::MAX, u32::MAX);
    }

    #[test]
    #[should_panic]
    fn test_tuple_addition_overflow_signed() {
        let coord = GridCoordinate::new(i32::MAX, i32::MAX);
        let _ = coord + (1, 1);
    }

    #[test]
    fn test_get_valid_coordinates() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let coord = GridCoordinate::new(1_usize, 1_usize);
        assert_eq!(coord.get(&grid).unwrap(), &5);

        let coord = GridCoordinate::new(0_usize, 2_usize);
        assert_eq!(coord.get(&grid).unwrap(), &3);

        let coord = GridCoordinate::new(2_usize, 0_usize);
        assert_eq!(coord.get(&grid).unwrap(), &7);
    }

    #[test]
    fn test_get_invalid_coordinates() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        // Test out of bounds
        let coord = GridCoordinate::new(3_usize, 1_usize);
        assert_eq!(coord.get(&grid), None);

        let coord = GridCoordinate::new(1_usize, 3_usize);
        assert_eq!(coord.get(&grid), None);

        // Test negative coordinates with signed integers
        let coord = GridCoordinate::new(-1_i32, 1_i32);
        assert_eq!(coord.get(&grid), None);
    }

    #[test]
    fn test_get_empty_grid() {
        let empty_grid: Vec<Vec<i32>> = Vec::new();
        let coord = GridCoordinate::new(0_usize, 0_usize);
        assert_eq!(coord.get(&empty_grid), None);

        let empty_rows: Vec<Vec<i32>> = vec![Vec::new()];
        assert_eq!(coord.get(&empty_rows), None);
    }

    mod shifted {
        use super::*;

        mod signed {
            use super::*;

            #[test]
            fn test_shifted_up() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Up),
                    Some(GridCoordinate { row: 0, column: 1 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate { row: 0, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Up),
                    Some(GridCoordinate { row: -1, column: 1 })
                );
            }

            #[test]
            fn test_shifted_down() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Down),
                    Some(GridCoordinate { row: 2, column: 1 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate { row: -1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Down),
                    Some(GridCoordinate { row: 0, column: 1 })
                );
            }

            #[test]
            fn test_shifted_left() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Left),
                    Some(GridCoordinate { row: 1, column: 0 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 0 };
                assert_eq!(
                    coord.shifted(GridDirection::Left),
                    Some(GridCoordinate { row: 1, column: -1 })
                );
            }

            #[test]
            fn test_shifted_right() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Right),
                    Some(GridCoordinate { row: 1, column: 2 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: -1 };
                assert_eq!(
                    coord.shifted(GridDirection::Right),
                    Some(GridCoordinate { row: 1, column: 0 })
                );
            }

            #[test]
            fn test_shifted_up_left() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::UpLeft),
                    Some(GridCoordinate { row: 0, column: 0 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate { row: 0, column: 0 };
                assert_eq!(
                    coord.shifted(GridDirection::UpLeft),
                    Some(GridCoordinate {
                        row: -1,
                        column: -1
                    })
                );
            }

            #[test]
            fn test_shifted_up_right() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::UpRight),
                    Some(GridCoordinate { row: 0, column: 2 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate { row: 0, column: -1 };
                assert_eq!(
                    coord.shifted(GridDirection::UpRight),
                    Some(GridCoordinate { row: -1, column: 0 })
                );
            }

            #[test]
            fn test_shifted_down_left() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::DownLeft),
                    Some(GridCoordinate { row: 2, column: 0 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate { row: -1, column: 0 };
                assert_eq!(
                    coord.shifted(GridDirection::DownLeft),
                    Some(GridCoordinate { row: 0, column: -1 })
                );
            }

            #[test]
            fn test_shifted_down_right() {
                let coord: GridCoordinate<i32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::DownRight),
                    Some(GridCoordinate { row: 2, column: 2 })
                );

                let coord: GridCoordinate<i32> = GridCoordinate {
                    row: -1,
                    column: -1,
                };
                assert_eq!(
                    coord.shifted(GridDirection::DownRight),
                    Some(GridCoordinate { row: 0, column: 0 })
                );
            }
        }

        mod unsigned {
            use super::*;

            #[test]
            fn test_shifted_up() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Up),
                    Some(GridCoordinate { row: 0, column: 1 })
                );

                let coord: GridCoordinate<u32> = GridCoordinate { row: 0, column: 1 };
                assert_eq!(coord.shifted(GridDirection::Up), None);
            }

            #[test]
            fn test_shifted_down() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Down),
                    Some(GridCoordinate { row: 2, column: 1 })
                );

                let coord: GridCoordinate<u32> = GridCoordinate { row: 0, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Down),
                    Some(GridCoordinate { row: 1, column: 1 })
                );
            }

            #[test]
            fn test_shifted_left() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Left),
                    Some(GridCoordinate { row: 1, column: 0 })
                );

                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 0 };
                assert_eq!(coord.shifted(GridDirection::Left), None);
            }

            #[test]
            fn test_shifted_right() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::Right),
                    Some(GridCoordinate { row: 1, column: 2 })
                );

                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 0 };
                assert_eq!(
                    coord.shifted(GridDirection::Right),
                    Some(GridCoordinate { row: 1, column: 1 })
                );
            }

            #[test]
            fn test_shifted_up_left() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::UpLeft),
                    Some(GridCoordinate { row: 0, column: 0 })
                );

                // Test boundary cases
                let coord: GridCoordinate<u32> = GridCoordinate { row: 0, column: 1 };
                assert_eq!(coord.shifted(GridDirection::UpLeft), None);

                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 0 };
                assert_eq!(coord.shifted(GridDirection::UpLeft), None);
            }

            #[test]
            fn test_shifted_up_right() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::UpRight),
                    Some(GridCoordinate { row: 0, column: 2 })
                );

                let coord: GridCoordinate<u32> = GridCoordinate { row: 0, column: 1 };
                assert_eq!(coord.shifted(GridDirection::UpRight), None);
            }

            #[test]
            fn test_shifted_down_left() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::DownLeft),
                    Some(GridCoordinate { row: 2, column: 0 })
                );

                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 0 };
                assert_eq!(coord.shifted(GridDirection::DownLeft), None);
            }

            #[test]
            fn test_shifted_down_right() {
                let coord: GridCoordinate<u32> = GridCoordinate { row: 1, column: 1 };
                assert_eq!(
                    coord.shifted(GridDirection::DownRight),
                    Some(GridCoordinate { row: 2, column: 2 })
                );

                let coord: GridCoordinate<u32> = GridCoordinate { row: 0, column: 0 };
                assert_eq!(
                    coord.shifted(GridDirection::DownRight),
                    Some(GridCoordinate { row: 1, column: 1 })
                );
            }
        }
    }

    mod surrounding_coordinates {
        use super::*;

        mod signed {
            use super::*;

            #[test]
            fn test_distance_one() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.surrounding_coordinates(1).collect();

                assert_eq!(coords.len(), 4);
                assert!(coords.contains(&GridCoordinate::new(4, 5))); // left
                assert!(coords.contains(&GridCoordinate::new(6, 5))); // right
                assert!(coords.contains(&GridCoordinate::new(5, 4))); // up
                assert!(coords.contains(&GridCoordinate::new(5, 6))); // down
            }

            #[test]
            fn test_distance_two() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.surrounding_coordinates(2).collect();

                assert_eq!(coords.len(), 12);
                assert!(coords.contains(&GridCoordinate::new(3, 5))); // 2 left
                assert!(coords.contains(&GridCoordinate::new(5, 3))); // 2 up
                assert!(coords.contains(&GridCoordinate::new(4, 4))); // diagonal
            }

            #[test]
            fn test_zero_distance() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.surrounding_coordinates(0).collect();
                assert_eq!(coords, vec![]);
            }

            #[test]
            fn test_doesnt_include_self() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.surrounding_coordinates(1).collect();
                assert!(!coords.contains(&center));
            }
        }

        mod unsigned {
            use super::*;

            #[test]
            fn test_near_zero() {
                let center = GridCoordinate::new(1u32, 1u32);
                let coords: Vec<_> = center.surrounding_coordinates(1).collect();

                assert_eq!(coords.len(), 4);
                assert!(coords.contains(&GridCoordinate::new(2, 1))); // down
                assert!(coords.contains(&GridCoordinate::new(1, 2))); // right
                assert!(coords.contains(&GridCoordinate::new(1, 0))); // left
                assert!(coords.contains(&GridCoordinate::new(0, 1))); // up
            }

            #[test]
            fn test_from_origin() {
                let origin = GridCoordinate::new(0u32, 0u32);
                let coords: Vec<_> = origin.surrounding_coordinates(1).collect();

                assert_eq!(coords.len(), 2); // Only right and down possible
                assert!(coords.contains(&GridCoordinate::new(1, 0))); // down
                assert!(coords.contains(&GridCoordinate::new(0, 1))); // right
            }

            #[test]
            fn test_distance_two_from_origin() {
                let origin = GridCoordinate::new(0u32, 0u32);
                let coords: Vec<_> = origin.surrounding_coordinates(2).collect();

                assert_eq!(coords.len(), 5); // Only positive coordinates
                assert!(coords.contains(&GridCoordinate::new(2, 0))); // 2 down
                assert!(coords.contains(&GridCoordinate::new(0, 2))); // 2 right
                assert!(coords.contains(&GridCoordinate::new(1, 1))); // diagonal
            }

            #[test]
            fn test_away_from_zero() {
                let center = GridCoordinate::new(5u32, 5u32);
                let coords: Vec<_> = center.surrounding_coordinates(1).collect();

                assert_eq!(coords.len(), 4); // All coordinates possible
                assert!(coords.contains(&GridCoordinate::new(4, 5)));
                assert!(coords.contains(&GridCoordinate::new(6, 5)));
                assert!(coords.contains(&GridCoordinate::new(5, 4)));
                assert!(coords.contains(&GridCoordinate::new(5, 6)));
            }
        }
    }

    mod edge_coordinates {
        use super::*;

        mod signed {
            use super::*;

            #[test]
            fn test_distance_one() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.edge_coordinates(1).collect();

                assert_eq!(coords.len(), 4);
                assert!(coords.contains(&GridCoordinate::new(4, 5)));
                assert!(coords.contains(&GridCoordinate::new(6, 5)));
                assert!(coords.contains(&GridCoordinate::new(5, 4)));
                assert!(coords.contains(&GridCoordinate::new(5, 6)));
            }

            #[test]
            fn test_distance_two() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.edge_coordinates(2).collect();

                assert_eq!(coords.len(), 8);
                assert!(coords.contains(&GridCoordinate::new(3, 5))); // 2 left
                assert!(coords.contains(&GridCoordinate::new(5, 3))); // 2 up
                assert!(coords.contains(&GridCoordinate::new(4, 4))); // up-left
                assert!(coords.contains(&GridCoordinate::new(6, 4))); // up-right
            }

            #[test]
            fn test_zero_distance() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.edge_coordinates(0).collect();
                assert_eq!(coords, vec![]);
            }

            #[test]
            fn test_doesnt_include_self() {
                let center = GridCoordinate::new(5i32, 5i32);
                let coords: Vec<_> = center.edge_coordinates(1).collect();
                assert!(!coords.contains(&center));
            }
        }

        mod unsigned {
            use super::*;

            #[test]
            fn test_near_zero() {
                let center = GridCoordinate::new(1u32, 1u32);
                let coords: Vec<_> = center.edge_coordinates(1).collect();

                assert_eq!(coords.len(), 4);
                assert!(coords.contains(&GridCoordinate::new(2, 1))); // down
                assert!(coords.contains(&GridCoordinate::new(1, 2))); // right
                assert!(coords.contains(&GridCoordinate::new(1, 0))); // left
                assert!(coords.contains(&GridCoordinate::new(0, 1))); // up
            }

            #[test]
            fn test_from_origin() {
                let origin = GridCoordinate::new(0u32, 0u32);
                let coords: Vec<_> = origin.edge_coordinates(1).collect();

                assert_eq!(coords.len(), 2); // Only right and down possible
                assert!(coords.contains(&GridCoordinate::new(1, 0))); // down
                assert!(coords.contains(&GridCoordinate::new(0, 1))); // right
            }

            #[test]
            fn test_distance_two_from_origin() {
                let origin = GridCoordinate::new(0u32, 0u32);
                let coords: Vec<_> = origin.edge_coordinates(2).collect();

                assert_eq!(coords.len(), 3); // Only positive coordinates on edge
                assert!(coords.contains(&GridCoordinate::new(2, 0))); // 2 down
                assert!(coords.contains(&GridCoordinate::new(0, 2))); // 2 right
                assert!(coords.contains(&GridCoordinate::new(1, 1))); // diagonal
            }

            #[test]
            fn test_away_from_zero() {
                let center = GridCoordinate::new(5u32, 5u32);
                let coords: Vec<_> = center.edge_coordinates(1).collect();

                assert_eq!(coords.len(), 4); // All coordinates possible
                assert!(coords.contains(&GridCoordinate::new(4, 5)));
                assert!(coords.contains(&GridCoordinate::new(6, 5)));
                assert!(coords.contains(&GridCoordinate::new(5, 4)));
                assert!(coords.contains(&GridCoordinate::new(5, 6)));
            }
        }
    }
}
