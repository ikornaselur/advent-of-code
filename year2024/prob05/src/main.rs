use advent::prelude::*;
use parse::{parse_input, PageOrder};
use std::cmp::Ordering;

mod parse;

fn main() -> Result<()> {
    let input = get_input(2024, 5)?;

    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, &input)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, &input)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        &input,
    );

    Ok(())
}

/// Create a map of pages that must be before a given page
///
/// The key is a page and the value is a list of values that must come after it, this can be used
/// to see if we have seen any of the 'after' pages before the key page, which means the order is invalid
fn page_before_map(page_orders: Vec<PageOrder>) -> HashMap<usize, HashSet<usize>> {
    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (before, after) in page_orders {
        if let Some(set) = map.get_mut(&before) {
            set.insert(after);
        } else {
            map.insert(before, HashSet::from_iter(vec![after]));
        }
    }
    map
}

fn part1(input: &str) -> Result<usize> {
    let (page_orders, page_lists) = parse_input(input)?;
    let before_map = page_before_map(page_orders);

    let mut return_val = 0;
    'pages: for page_list in page_lists {
        let mut seen = HashSet::new();
        for digit in &page_list {
            // Check if any of the 'seen' digits are in the 'after' values
            if let Some(after) = before_map.get(digit) {
                if after.intersection(&seen).count() > 0 {
                    continue 'pages;
                }
            }
            seen.insert(*digit);
        }

        // We reached the end without any issues
        // Now get the middle value of the page_list
        return_val += page_list.get(page_list.len() / 2).unwrap();
    }
    Ok(return_val)
}

fn part2(input: &str) -> Result<usize> {
    let (page_orders, page_lists) = parse_input(input)?;
    let before_map = page_before_map(page_orders);

    let mut invalid_page_lists = Vec::new();
    'pages: for page_list in page_lists {
        let mut seen = HashSet::new();
        for digit in &page_list {
            // Check if any of the 'seen' digits are in the 'after' values
            if let Some(after) = before_map.get(digit) {
                if after.intersection(&seen).count() > 0 {
                    invalid_page_lists.push(page_list);
                    continue 'pages;
                }
            }
            seen.insert(*digit);
        }
    }

    // Go through the invalid pages and sort them based on the before_map
    let sum = invalid_page_lists
        .iter()
        .map(|page_list| {
            let mut sorted = page_list.clone();
            sorted.sort_by(|a, b| {
                // Check if a needs to be before b
                if let Some(after) = before_map.get(a) {
                    if after.contains(b) {
                        return Ordering::Less;
                    }
                }
                // Check the other way
                if let Some(after) = before_map.get(b) {
                    if after.contains(a) {
                        return Ordering::Greater;
                    }
                }
                // Otherwise they're the same and it doesn't matter
                Ordering::Equal
            });
            // Then we return the middle value
            *sorted.get(sorted.len() / 2).unwrap()
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 123);
    }

    #[test]
    fn test_page_before_map() {
        let page_orders = vec![(1, 4), (3, 5), (3, 4)];
        let map = page_before_map(page_orders);
        assert_eq!(map.get(&1).unwrap(), &HashSet::from_iter(vec![4]));
        assert_eq!(map.get(&3).unwrap(), &HashSet::from_iter(vec![4, 5]));
    }
}
