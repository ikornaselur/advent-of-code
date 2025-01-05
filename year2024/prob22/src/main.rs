use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fs;

mod parse;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2024, 22)?
    };

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

fn part1(input: &str) -> Result<usize> {
    let digits = parse_input(input)?;

    Ok(digits
        .iter()
        .map(|digit| {
            let mut num = *digit;
            for _ in 0..2000 {
                num = steps(num);
            }
            num
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let digits = parse_input(input)?;
    let seller_prices = digits.iter().map(|&d| get_prices(d)).collect::<Vec<_>>();

    let mut total_bananas: HashMap<u32, usize> = HashMap::new();
    for prices in seller_prices {
        for (diff, bananas) in prices {
            *total_bananas.entry(diff).or_insert(0) += bananas as usize;
        }
    }

    Ok(total_bananas.values().max().copied().unwrap_or(0))
}

fn steps(num: usize) -> usize {
    let n1 = ((num * 64) ^ num) % 16_777_216;
    let n2 = ((n1 / 32) ^ n1) % 16_777_216;
    ((n2 * 2048) ^ n2) % 16_777_216
}

/// Calculate the prices for the first 2000 secret numbers
///
/// Since we need to instruct the monkey to buy with a 4-in-a-row diff of the last digit, what we
/// can do is keep a running "diff" number, which is just the last 4 diffs packed in a 32 bit
/// number.
///
/// We'll use some masking and bitshifting magic for this, then we store the current number of
/// bananas available at each diff. Then at the end, we can just test all the diffs we have across
/// all sellers to find the highest total bananas for a given diff.
fn get_prices(secret: usize) -> HashMap<u32, u8> {
    let mut diff: u32 = 0;
    let mut last_digit = (secret % 10) as u8;
    let mut secret = steps(secret);

    let total_steps = 2000;

    let mut prices = HashMap::new();
    for i in 0..total_steps - 1 {
        let digit: u8 = (secret % 10) as u8;
        secret = steps(secret);

        // This is some magic, we get the diff.. which will be from -9 to 9, then we just add 9 to
        // it to bring it to become 0 to 18. We'll pack this into the diff by shifting by 5 bits
        // and adding the new digit diff. We then use a 20 bit mask to only look at the last 20
        // bits, meaning we are looking at the last 4 diffs in a unique 20 bit number (stored as
        // u32)
        let last_digit_diff: u32 = ((digit as i8 - last_digit as i8) + 9) as u32;
        last_digit = digit;
        diff = ((diff << 5) | last_digit_diff) & 0xF_FF_FF;

        // We'll build up the diff with the first 4 secrets numbers, as we can only start matching
        // when we have 4 numbers
        if i < 4 {
            continue;
        }

        // Then we start storing values, but only the *first* time we see it, as the monkey wont be
        // skipping over diffs for later ones
        prices.entry(diff).or_insert(digit);
    }

    prices
}

#[allow(dead_code)]
fn pack_diffs(a: i8, b: i8, c: i8, d: i8) -> u32 {
    let a = ((a + 9) as u32) & 0x1F;
    let b = ((b + 9) as u32) & 0x1F;
    let c = ((c + 9) as u32) & 0x1F;
    let d = ((d + 9) as u32) & 0x1F;

    (a << 15) | (b << 10) | (c << 5) | d
}

#[allow(dead_code)]
fn unpack_diffs(packed: u32) -> (i8, i8, i8, i8) {
    let a = ((packed >> 15) & 0x1F) as i8 - 9;
    let b = ((packed >> 10) & 0x1F) as i8 - 9;
    let c = ((packed >> 5) & 0x1F) as i8 - 9;
    let d = (packed & 0x1F) as i8 - 9;
    (a, b, c, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const TEST_INPUT2: &str = include_str!("../test2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 37_327_623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT2).unwrap(), 23);
    }

    #[test]
    fn test_steps() {
        assert_eq!(steps(123), 15_887_950);
    }

    #[test]
    fn test_pack_diffs() {
        assert_eq!(pack_diffs(-1, -1, 0, 2), 0x04_21_2B);
    }

    #[test]
    fn test_unpack_diffs() {
        assert_eq!(unpack_diffs(0x04_21_2B), (-1, -1, 0, 2));
    }

    #[test]
    fn test_get_prices() {
        let prices = get_prices(123);

        let diff = pack_diffs(-1, -1, 0, 2);

        assert_eq!(prices.get(&diff).unwrap(), &6);
    }
}
