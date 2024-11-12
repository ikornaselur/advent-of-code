use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Default)]
struct DistanceMap {
    map: HashMap<(Coordinate<usize>, Coordinate<usize>), usize>,
}

impl DistanceMap {
    fn insert(&mut self, from: Coordinate<usize>, to: Coordinate<usize>, distance: usize) {
        if from > to {
            self.map.insert((to, from), distance);
        } else {
            self.map.insert((from, to), distance);
        }
    }
}

/// An image is a list of strings, which contain "." for empty space and "#" for a galaxy
struct Image {
    map: Vec<Vec<bool>>,
    row_scale: Vec<usize>,
    column_scale: Vec<usize>,
}

impl Image {
    /// Set the
    fn set_scale(&mut self, amount: usize) {
        self.row_scale = self
            .map
            .iter()
            .map(|row| row.iter().any(|&value| value))
            .map(|has_true| if has_true { 1 } else { amount })
            .collect();

        self.column_scale = (0..self.map[0].len())
            .map(|col_idx| self.map.iter().any(|row| row[col_idx]))
            .map(|has_true| if has_true { 1 } else { amount })
            .collect();
    }

    /// Get the distance map for this image
    ///
    /// The distance map is the shortest distance between all galaxies
    fn get_distance_map(&self) -> Result<DistanceMap> {
        let mut distance_map = DistanceMap::default();

        // Find all the galaxies
        let mut galaxy_coords: Vec<Coordinate<usize>> = vec![];
        for (y, row) in self.map.iter().enumerate() {
            for (x, &galaxy) in row.iter().enumerate() {
                if galaxy {
                    galaxy_coords.push((x, y));
                }
            }
        }

        // Then we start from each galaxy and calculate the distance to other galaxies
        // We can simply calculate the difference between the x and y coordinates
        //
        // Examples:
        //  if we have a galaxy at (0, 0) and (3, 3), the distance is 6
        //  if we have a galaxy at (2, 3) and (4, 6), the distance is 5
        for left_idx in 0..galaxy_coords.len() {
            for right_idx in left_idx + 1..galaxy_coords.len() {
                let left_coord = galaxy_coords[left_idx];
                let right_coord = galaxy_coords[right_idx];
                // Check if we would cross any nodes that are scaled
                let mut distance = 0;

                let (mut x, x_end) = if left_coord.0 < right_coord.0 {
                    (left_coord.0, right_coord.0)
                } else {
                    (right_coord.0, left_coord.0)
                };
                while x < x_end {
                    distance += self.column_scale[x];
                    x += 1;
                }

                let (mut y, y_end) = if left_coord.1 < right_coord.1 {
                    (left_coord.1, right_coord.1)
                } else {
                    (right_coord.1, left_coord.1)
                };
                while y < y_end {
                    distance += self.row_scale[y];
                    y += 1;
                }

                distance_map.insert(left_coord, right_coord, distance);
            }
        }

        Ok(distance_map)
    }
}

impl FromStr for Image {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let map: Vec<_> = s
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        Ok(Image {
            map: map.clone(),
            row_scale: vec![1; map.len()],
            column_scale: vec![1; map[0].len()],
        })
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut image: Image = input.parse()?;

    image.set_scale(2);

    let distance_map = image.get_distance_map()?;

    Ok(distance_map.map.values().sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut image: Image = input.parse()?;

    image.set_scale(1000000);

    let distance_map = image.get_distance_map()?;

    Ok(distance_map.map.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 374);
    }

    #[test]
    fn test_part2() {
        let mut image: Image = TEST_INPUT.parse().unwrap();

        image.set_scale(100);

        let distance_map = image.get_distance_map().unwrap();

        assert_eq!(distance_map.map.values().sum::<usize>(), 8410)
    }

    #[test]
    fn test_image_from_str() {
        let image: Image = TEST_INPUT.parse().unwrap();

        assert_eq!(image.map.len(), 10);
        assert_eq!(
            image.map[0],
            vec![false, false, false, true, false, false, false, false, false, false]
        );
    }

    #[test]
    fn test_image_expand_base_test() {
        // ...
        // #..
        // ..#
        let mut image: Image = "...\n#..\n..#".parse().unwrap();

        assert_eq!(image.row_scale, vec![1, 1, 1]);
        assert_eq!(image.column_scale, vec![1, 1, 1]);

        image.set_scale(2);

        assert_eq!(image.row_scale, vec![2, 1, 1]);
        assert_eq!(image.column_scale, vec![1, 2, 1]);
    }

    #[test]
    fn test_image_expand_bigger_input() {
        let mut image: Image = TEST_INPUT.parse().unwrap();

        assert_eq!(image.row_scale, vec![1; 10]);
        assert_eq!(image.column_scale, vec![1; 10]);

        image.set_scale(2);

        assert_eq!(image.row_scale, vec![1, 1, 1, 2, 1, 1, 1, 2, 1, 1]);
        assert_eq!(image.column_scale, vec![1, 1, 2, 1, 1, 2, 1, 1, 2, 1]);
    }

    #[test]
    fn test_distance_map_inserts_by_sorted_tuple() {
        let mut distance_map = DistanceMap::default();

        distance_map.insert((0, 0), (1, 0), 1);
        distance_map.insert((1, 0), (0, 0), 1);

        assert_eq!(distance_map.map.len(), 1);
    }
}
