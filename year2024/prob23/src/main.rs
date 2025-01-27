use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fmt;
use std::fs;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Computer {
    id: (char, char),
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.id.0, self.id.1)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2024, 23)?
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
    let connections = parse_input(input)?;

    let mut groups = HashMap::new();
    for (from, to) in &connections {
        groups.entry(from).or_insert_with(HashSet::new).insert(to);
        groups.entry(to).or_insert_with(HashSet::new).insert(from);
    }

    // Let's find a set of three connected computers, where each node is connected to the other
    // two only. We can do this by iterating through all the nodes, and checking if the original
    // node is connected to any of the connected nodes... does this make sense?
    // So for node A, we go through it's connected nodes, which we can call node B (that is, A->B)
    // and then iterate all the nodes connected to B. Those nodes we can call C (that is, B->C).
    // For each node C, we check if it's connected to A, which means we have a A->B->C->A (three
    // nodes connect to each other)
    //
    // We'll be storing these triples as sorted tuples, so prevent duplicates (as A,B,C is the same
    // as C,B,A in terms of a node grouping)
    //
    // Additionally.. we only care about triples which contain a node that starts with 't' in part 1
    let mut triples = HashSet::new();

    // For simplicity, based on the description above
    // A: from
    // B: to
    // C: next
    for (from, to) in &connections {
        for next in groups.get(to).unwrap() {
            // Skip if none of the nodes start with 't'
            if from.id.0 != 't' && to.id.0 != 't' && next.id.0 != 't' {
                continue;
            }
            if groups.get(next).unwrap().contains(from) {
                let mut triple = vec![from, to, next];
                triple.sort();
                triples.insert(triple);
            }
        }
    }

    Ok(triples.len())
}

fn part2(input: &str) -> Result<String> {
    let connections = parse_input(input)?;

    // This time we're searching for the largest clique (just learned that term) in this graph
    //
    // Based on the Wiki article about the 'clique problem', a simple greedy algo should be fine to
    // find a single maximal clique.. so let's try that

    let mut adjacency = HashMap::new();
    for (from, to) in &connections {
        adjacency
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
        adjacency
            .entry(to)
            .or_insert_with(HashSet::new)
            .insert(from);
    }

    let mut largest_clique: Vec<&Computer> = Vec::new();

    for (&node, neighbours) in adjacency.iter() {
        let mut candidate_clique: Vec<&Computer> = vec![node];

        for &neighbour in neighbours {
            if candidate_clique
                .iter()
                .all(|&n| adjacency[n].contains(neighbour))
            {
                candidate_clique.push(neighbour);
            }
        }

        if candidate_clique.len() > largest_clique.len() {
            largest_clique = candidate_clique.clone();
        }
    }

    largest_clique.sort_unstable();

    Ok(largest_clique
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), String::from("co,de,ka,ta"));
    }
}
