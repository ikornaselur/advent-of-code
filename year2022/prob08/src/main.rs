use advent::prelude::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Visibility {
    left: i32,
    right: i32,
    above: i32,
    below: i32,
}

impl Visibility {
    fn is_visible(&self, tree: &Tree) -> bool {
        tree.height > self.left
            || tree.height > self.right
            || tree.height > self.above
            || tree.height > self.below
    }
}

#[derive(Debug)]
struct Tree {
    height: i32,
    visibility: Visibility,
    visible: Option<bool>,
}

impl Tree {
    fn new(height: i32) -> Self {
        Self {
            height,
            visible: None,
            visibility: Visibility {
                left: -1,
                right: -1,
                above: -1,
                below: -1,
            },
        }
    }
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    /// Go through the trees in the forest and check if they are visible from the outside
    ///
    /// A tree is visible if:
    ///
    /// 1. It's on an edge of the forest
    /// 2. It's taller than it's neighbours
    /// 3. We only care about the four main directions, left, right, above and below
    ///
    /// We can do this by doing doing four passes, one for each direction. We we go through the
    /// trees in each pass and update the max height we've seen so far from the direction we are
    /// coming from. When we've done that from all directions, we can go through each tree and see
    /// if it's bigger than any 'direction height'
    fn check_visibility(&mut self) {
        // From left + right
        for row in 0..self.trees.len() {
            // Left
            let mut max_height = -1;
            for col in 0..self.trees[row].len() {
                let tree = &mut self.trees[row][col];
                tree.visibility.left = max_height;
                if tree.height > max_height {
                    max_height = tree.height;
                }
            }

            // Right
            let mut max_height = -1;
            for col in (0..self.trees[row].len()).rev() {
                let tree = &mut self.trees[row][col];
                tree.visibility.right = max_height;
                if tree.height > max_height {
                    max_height = tree.height;
                }
            }
        }

        // Form above and below
        for col in 0..self.trees[0].len() {
            // Above
            let mut max_height = -1;
            for row in 0..self.trees.len() {
                let tree = &mut self.trees[row][col];
                tree.visibility.above = max_height;
                if tree.height > max_height {
                    max_height = tree.height;
                }
            }
            // Below
            let mut max_height = -1;
            for row in (0..self.trees.len()).rev() {
                let tree = &mut self.trees[row][col];
                tree.visibility.below = max_height;
                if tree.height > max_height {
                    max_height = tree.height;
                }
            }
        }

        // Finally, set the visible flag or all trees
        self.trees.iter_mut().flatten().for_each(|tree| {
            tree.visible = Some(tree.visibility.is_visible(tree));
        });
    }

    /// Calculate the scenic score at a coordinate
    ///
    /// A scenic score is a multiple of how many trees can be seen from each side
    /// This is done by traversing in all directions, counting seen tree, until hitting either an
    /// edge or a tree that is of equal height or higher. Any trees past that do not matter.
    /// The total count is then multiplied, giving a scenic score
    fn get_tree_scenic_score(&self, x: usize, y: usize) -> usize {
        let tree = &self.trees[x][y];
        let height = tree.height;

        // Keep the counts, in the order of left, right, above, below
        let mut tree_counts = [0; 4];

        // Check left side:
        for col in (0..y).rev() {
            tree_counts[0] += 1;
            if self.trees[x][col].height >= height {
                break;
            }
        }

        // Check right side:
        for col in y + 1..self.trees[x].len() {
            tree_counts[1] += 1;
            if self.trees[x][col].height >= height {
                break;
            }
        }

        // Check above:
        for row in (0..x).rev() {
            tree_counts[2] += 1;
            if self.trees[row][y].height >= height {
                break;
            }
        }

        // Check below:
        for row in x + 1..self.trees.len() {
            tree_counts[3] += 1;
            if self.trees[row][y].height >= height {
                break;
            }
        }

        tree_counts.iter().product()
    }
}

impl FromStr for Forest {
    type Err = AdventError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let trees = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .ok_or(AdventError::InvalidDigit(c))
                            .map(|height| Tree::new(height as i32))
                    })
                    .collect::<std::result::Result<Vec<Tree>, AdventError>>()
            })
            .collect::<std::result::Result<Vec<Vec<Tree>>, AdventError>>()?;

        Ok(Self { trees })
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
    let mut forest: Forest = input.parse()?;
    forest.check_visibility();

    Ok(forest
        .trees
        .iter()
        .flatten()
        .filter(|tree| tree.visible == Some(true))
        .count())
}

fn part2(input: &str) -> Result<usize> {
    let forest: Forest = input.parse()?;

    let row_count = forest.trees.len();
    let col_count = forest.trees[0].len();

    (0..row_count)
        .flat_map(|x| (0..col_count).map(move |y| (x, y)))
        .map(|(x, y)| forest.get_tree_scenic_score(x, y))
        .max()
        .ok_or(error!("Unable to find max scenic score"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 8);
    }

    #[test]
    fn test_forest_from_str() {
        let input = "123\n456\n789";
        let forest: Forest = input.parse().unwrap();

        assert_eq!(forest.trees.len(), 3);
        assert_eq!(forest.trees[0].len(), 3);
        assert_eq!(forest.trees[1].len(), 3);
        assert_eq!(forest.trees[2].len(), 3);

        assert_eq!(forest.trees[0][0].height, 1);
        assert_eq!(forest.trees[1][1].height, 5);
        assert_eq!(forest.trees[2][2].height, 9);
        assert_eq!(forest.trees[0][2].height, 3);
        assert_eq!(forest.trees[2][0].height, 7);
    }

    #[test]
    fn test_forest_check_visibility() {
        let input = "163\n756\n789";
        let mut forest: Forest = input.parse().unwrap();

        forest.check_visibility();

        assert_eq!(forest.trees[0][0].visibility.left, -1);
        assert_eq!(forest.trees[0][0].visibility.right, 6);
        assert_eq!(forest.trees[0][0].visibility.above, -1);
        assert_eq!(forest.trees[0][0].visibility.below, 7);
        assert_eq!(forest.trees[0][0].visible, Some(true));

        assert_eq!(forest.trees[1][1].visibility.left, 7);
        assert_eq!(forest.trees[1][1].visibility.right, 6);
        assert_eq!(forest.trees[1][1].visibility.above, 6);
        assert_eq!(forest.trees[1][1].visibility.below, 8);
        assert_eq!(forest.trees[1][1].visible, Some(false));
    }

    #[test]
    fn test_forest_scenic_score() {
        let forest: Forest = TEST_INPUT.parse().unwrap();

        assert_eq!(forest.get_tree_scenic_score(1, 2), 4);
        assert_eq!(forest.get_tree_scenic_score(3, 2), 8);
        assert_eq!(forest.get_tree_scenic_score(0, 2), 0);
    }
}
