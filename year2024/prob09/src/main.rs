#![allow(clippy::comparison_chain)]
use advent::prelude::*;
use parse::parse_input;

mod parse;

#[derive(Debug, Copy, Clone, PartialEq)]
struct File {
    id: u32,
    size: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Node {
    Free(u32),
    File(File),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CompressedNode {
    Free(u32),
    File(u32),
}

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, INPUT)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, INPUT)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        INPUT,
    );

    Ok(())
}

fn expand_disk_map(compressed_disk_map: Vec<CompressedNode>) -> Vec<Node> {
    compressed_disk_map
        .iter()
        .scan(0, |file_id, compressed_node| match compressed_node {
            CompressedNode::File(size) => {
                let file = Node::File(File {
                    id: *file_id,
                    size: *size,
                });
                *file_id += 1;
                Some(file)
            }
            CompressedNode::Free(size) => Some(Node::Free(*size)),
        })
        .collect()
}

fn defrag_disk(disk_map: &mut Vec<Node>) {
    let mut idx = 0;

    while idx < disk_map.len() {
        //print_disk_map(disk_map.to_vec());
        if let Node::File(_) = disk_map[idx] {
            idx += 1;
            continue;
        }

        if let Node::Free(free_space) = disk_map[idx] {
            // Get the last item
            let last_item = disk_map.pop().unwrap();

            if let Node::File(file) = &last_item {
                if file.size < free_space {
                    // W+e move the whole file, there will still be some free space
                    disk_map.insert(idx, Node::File(*file));

                    idx += 1;
                    // Need to update the free space
                    disk_map[idx] = Node::Free(free_space - file.size);
                } else if file.size == free_space {
                    // Same as before, except it replaces the free space completely
                    disk_map[idx] = Node::File(*file);

                    idx += 1;
                } else {
                    // We've got to split the file, so we'll replace the free space with a chunk of
                    // the file, leaving the rest at the end for the next loop
                    disk_map[idx] = Node::File(File {
                        id: file.id,
                        size: free_space,
                    });

                    // Push the rest back to the end
                    // TODO: Could be optimised by just storing this?
                    disk_map.push(Node::File(File {
                        id: file.id,
                        size: file.size - free_space,
                    }));

                    idx += 1;
                }
            }
        } else {
            panic!("Invalid disk");
        }
    }
}

#[allow(dead_code)]
fn print_disk_map(disk_map: Vec<Node>) {
    for node in disk_map {
        match node {
            Node::Free(size) => print!("{}", ".".repeat(size as usize)),
            Node::File(file) => print!("{}", format!("{}", file.id).repeat(file.size as usize)),
        }
    }
    println!();
}

fn calculate_checksum(disk_map: Vec<Node>) -> u64 {
    let mut idx: u64 = 0;
    let mut checksum: u64 = 0;

    for node in disk_map {
        match node {
            Node::Free(_) => {
                // Since we just left free space probably at the end, we can just break here
                break;
            }
            Node::File(File { id, size }) => {
                for i in idx..(idx + size as u64) {
                    checksum += (id as u64) * i;
                }
                idx += size as u64;
            }
        }
    }

    checksum
}

fn part1(input: &str) -> Result<u64> {
    let compressed_disk_map = parse_input(input)?;
    let mut disk_map = expand_disk_map(compressed_disk_map);
    defrag_disk(&mut disk_map);

    Ok(calculate_checksum(disk_map))
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_expand_disk_map() {
        let disk_map = vec![
            CompressedNode::File(1),
            CompressedNode::Free(2),
            CompressedNode::File(3),
            CompressedNode::Free(4),
            CompressedNode::File(5),
        ];
        let expanded = expand_disk_map(disk_map);
        assert_eq!(
            expanded,
            vec![
                Node::File(File { id: 0, size: 1 }),
                Node::Free(2),
                Node::File(File { id: 1, size: 3 }),
                Node::Free(4),
                Node::File(File { id: 2, size: 5 }),
            ]
        );
    }
}
