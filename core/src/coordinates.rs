use num_traits::{FromPrimitive, PrimInt, Zero};
use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridCoordinate<T> {
    pub row: T,
    pub column: T,
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

impl<T: PrimInt> AddAssign<(T, T)> for GridCoordinate<T> {
    fn add_assign(&mut self, other: (T, T)) {
        self.row = self.row + other.0;
        self.column = self.column + other.1;
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
        assert_eq!(coord.get(&grid), Some(&5));

        let coord = GridCoordinate::new(0_usize, 2_usize);
        assert_eq!(coord.get(&grid), Some(&3));

        let coord = GridCoordinate::new(2_usize, 0_usize);
        assert_eq!(coord.get(&grid), Some(&7));
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
}
