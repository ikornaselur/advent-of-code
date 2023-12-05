use advent_core::error::AdventError;


const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    source_start: u64,
    destination_start: u64,
    range_length: u64,
}

impl Mapping {
    fn convert_number(&self, number: u64) -> u64 {
        if number >= self.source_start && number < self.source_start + self.range_length {
            self.destination_start + (number - self.source_start)
        } else {
            number
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    from: String,
    to: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn convert_number(&self, number: u64) -> u64 {
        for mapping in &self.mappings {
            let mapped_number = mapping.convert_number(number);
            if mapped_number != number {
                return mapped_number;
            }
        }
        number
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<(u64, u64)>,
    maps: Vec<Map>,
}

impl Almanac {
    /// Parse input into an Almanac
    ///
    /// The input will start with a line listing the seeds, as such:
    ///
    ///     seeds: 1 12 41 678
    ///
    /// followed by a list of maps, each separated by an empty new line:
    ///
    ///     x-to-y map:
    ///     10 20 4
    ///     31 6 21
    ///
    /// each map can have any number of mappings listed under the title
    fn from_str(input: &str) -> Result<Self, AdventError> {
        let mut lines = input.lines();

        // Get the seeds first
        let seeds: Vec<_> = lines
            .next()
            .ok_or(AdventError::InvalidInput)?
            .strip_prefix("seeds: ")
            .ok_or(AdventError::InvalidInput)?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        lines.next(); // Skip the empty line

        let mut maps = vec![];

        while let Some(line) = lines.next() {
            // Get the map name
            let map_name: Vec<_> = line
                .strip_suffix(" map:")
                .ok_or(AdventError::InvalidInput)?
                .split('-')
                .collect();
            let map_from = map_name[0];
            let map_to = map_name[2];

            // Get the mappings
            let mut mappings = vec![];
            for line in lines.by_ref() {
                if line.is_empty() {
                    // We've reached the end of the map
                    break;
                }
                let mapping: Vec<_> = line.split_whitespace().collect();
                let mapping = Mapping {
                    source_start: mapping[1].parse()?,
                    destination_start: mapping[0].parse()?,
                    range_length: mapping[2].parse()?,
                };
                mappings.push(mapping);
            }

            maps.push(Map {
                from: map_from.to_string(),
                to: map_to.to_string(),
                mappings,
            });
        }

        // Calculate the seed ranges
        let mut seed_ranges = vec![];

        let seed_iter = seeds.iter();
        for (range_start, range_len) in seed_iter.clone().zip(seed_iter.skip(1)).step_by(2) {
            seed_ranges.push((*range_start, *range_len));
        }

        Ok(Self { seeds, seed_ranges, maps })
    }

    /// Convert a number through all the mappings of the almanac
    fn convert_number(&self, number: u64) -> u64 {
        let mut number = number;
        for map in &self.maps {
            number = map.convert_number(number);
        }
        number
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u64, AdventError> {
    let almanac = Almanac::from_str(input)?;

    let mut lowest_number = u64::MAX;

    for seed in &almanac.seeds {
        let converted_seed = almanac.convert_number(*seed);
        if converted_seed < lowest_number {
            lowest_number = converted_seed;
        }
    }

    Ok(lowest_number)
}

fn part2(input: &str) -> Result<u64, AdventError> {
    let almanac = Almanac::from_str(input)?;

    let mut lowest_number = u64::MAX;

    for (seed_start, range_length) in &almanac.seed_ranges {
        for number in *seed_start..seed_start + range_length {
            let converted_number = almanac.convert_number(number);
            if converted_number < lowest_number {
                lowest_number = converted_number;
            }
        }
    }

    Ok(lowest_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 46);
    }

    #[test]
    fn test_alamanac_from_str() {
        let input = "seeds: 1 12 41 678\n\nfoo-to-bar map:\n10 20 4\n31 6 21\n\nbar-to-baz map:\n1 2 3\n4 5 6";

        let almanac = Almanac::from_str(input).unwrap();

        assert_eq!(almanac.seeds, vec![1, 12, 41, 678]);
        assert_eq!(almanac.maps.len(), 2);

        let map1 = &almanac.maps[0];
        assert_eq!(map1.from, "foo");
        assert_eq!(map1.to, "bar");

        assert_eq!(map1.mappings.len(), 2);
        assert_eq!(
            map1.mappings[0],
            Mapping {
                source_start: 20,
                destination_start: 10,
                range_length: 4
            }
        );
        assert_eq!(
            map1.mappings[1],
            Mapping {
                source_start: 6,
                destination_start: 31,
                range_length: 21
            }
        );

        let map2 = &almanac.maps[1];
        assert_eq!(map2.from, "bar");
        assert_eq!(map2.to, "baz");
        assert_eq!(map2.mappings.len(), 2);
        assert_eq!(
            map2.mappings[0],
            Mapping {
                source_start: 2,
                destination_start: 1,
                range_length: 3
            }
        );
        assert_eq!(
            map2.mappings[1],
            Mapping {
                source_start: 5,
                destination_start: 4,
                range_length: 6
            }
        );
    }

    #[test]
    fn test_mapping_convert_number() {
        let mapping = Mapping {
            source_start: 10,
            destination_start: 20,
            range_length: 4,
        };

        assert_eq!(mapping.convert_number(9), 9);
        assert_eq!(mapping.convert_number(10), 20);
        assert_eq!(mapping.convert_number(11), 21);
        assert_eq!(mapping.convert_number(12), 22);
        assert_eq!(mapping.convert_number(13), 23);
        assert_eq!(mapping.convert_number(14), 14);
    }

    #[test]
    fn test_convert_number() {
        let almanac = Almanac::from_str(TEST_INPUT).unwrap();

        assert_eq!(almanac.convert_number(79), 82);
    }
}
