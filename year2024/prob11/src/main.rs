use advent::prelude::*;
use parse::parse_input;

mod parse;

fn main() -> Result<()> {
    let input = get_input(2024, 11)?;

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

fn stone_tick(stone: usize) -> Result<(usize, Option<usize>)> {
    let dig_count = stone.checked_ilog10().unwrap_or(0) + 1;
    if stone == 0 {
        // Rule 1: 0 just turns into 1
        Ok((1, None))
    } else if dig_count % 2 == 0 {
        // Rule 2: If the stone has even number of digits, we replace it with two stones. Left half
        // of the digits on the left stone and right half of the digits to the right stone.
        // Note that leading zeros are ignored.
        let divisor = 10_usize.pow(dig_count / 2);
        Ok((stone / divisor, Some(stone % divisor)))
    } else {
        // Rule 3: Multiply the stone number with 2024
        Ok((stone * 2024, None))
    }
}

fn get_stone_count_after_ticks(
    stone: usize,
    ticks: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if ticks == 0 {
        // TODO: Is this base case correct?
        // Returning 1 because, this is just one stone?
        return 1;
    }

    let key = (stone, ticks);
    if let Some(cached) = cache.get(&key) {
        // If we've already cached the count, we can just return it
        return *cached;
    }

    match stone_tick(stone) {
        Ok((new_stone, None)) => {
            let count = get_stone_count_after_ticks(new_stone, ticks - 1, cache);
            cache.insert(key, count);
            count
        }
        Ok((left_stone, Some(right_stone))) => {
            let count = get_stone_count_after_ticks(left_stone, ticks - 1, cache)
                + get_stone_count_after_ticks(right_stone, ticks - 1, cache);

            cache.insert(key, count);
            count
        }
        _ => panic!("Invalid stone"),
    }
}

fn part1(input: &str) -> Result<usize> {
    let stones = parse_input(input)?;
    let steps = 25;

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut count = 0;

    for stone in stones {
        let stone_count = get_stone_count_after_ticks(stone, steps, &mut cache);
        count += stone_count;
    }

    Ok(count)
}

fn part2(input: &str) -> Result<usize> {
    let stones = parse_input(input)?;
    let steps = 75;

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut count = 0;

    for stone in stones {
        let stone_count = get_stone_count_after_ticks(stone, steps, &mut cache);
        count += stone_count;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 65_601_038_650_482);
    }

    #[test]
    fn test_stone_tick_rule1() {
        assert_eq!(stone_tick(0).unwrap(), (1, None));
    }

    #[test]
    fn test_stone_tick_rule2() {
        assert_eq!(stone_tick(10).unwrap(), (1, Some(0)));
        assert_eq!(stone_tick(1000).unwrap(), (10, Some(0)));
        assert_eq!(stone_tick(1234).unwrap(), (12, Some(34)));
    }

    #[test]
    fn test_stone_tick_rule3() {
        assert_eq!(stone_tick(1).unwrap(), (2024, None));
    }

    #[test]
    fn test_get_stone_count_after_ticks() {
        let mut cache = HashMap::new();

        // 0 will just be 1 after one tick
        cache.clear();
        assert_eq!(get_stone_count_after_ticks(0, 1, &mut cache), 1);

        // 1 will become 2024 after one tick
        cache.clear();
        assert_eq!(get_stone_count_after_ticks(1, 1, &mut cache), 1);

        // 2024 will become 20 and 24 after one tick
        cache.clear();
        assert_eq!(get_stone_count_after_ticks(2024, 1, &mut cache), 2);

        // 0 should become 4 stones after 5 ticks
        cache.clear();
        assert_eq!(get_stone_count_after_ticks(0, 5, &mut cache), 4);

        // 0 should become 7 stones after 6 ticks
        cache.clear();
        assert_eq!(get_stone_count_after_ticks(0, 6, &mut cache), 7);
    }
}
