use advent::prelude::*;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .fold((0, 0), |(acc, max), val| {
            let new_acc = match val {
                "" => 0,
                _ => acc + val.parse::<u32>().unwrap(),
            };

            (new_acc, max.max(new_acc))
        })
        .1)
}

fn part2(input: &str) -> Result<u32> {
    let mut heap = BinaryHeap::new();
    let mut acc = 0;
    for val in input.lines() {
        match val {
            "" => {
                heap.push(0 - acc); // BinaryHeap is a max-heap, we want a min-heap
                if heap.len() > 3 {
                    heap.pop(); // Get rid of the smallest
                }
                acc = 0;
            }
            _ => {
                acc += val.parse::<i32>()?;
            }
        }
    }

    // Add the last group
    heap.push(0 - acc);
    if heap.len() > 3 {
        heap.pop(); // Get rid of the smallest
    }

    Ok(heap.iter().sum::<i32>().unsigned_abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 24_000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 45000);
    }
}
