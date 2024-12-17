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

fn main() -> Result<()> {
    let input = get_input(2024, 9)?;

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

fn defrag_disk(disk_map: &mut Vec<Node>, split_files: bool) {
    let mut left_idx = 0;
    let mut right_idx = disk_map.len() - 1;

    while left_idx < right_idx {
        if let Node::File(_) = disk_map[left_idx] {
            left_idx += 1;
            continue;
        }

        if let Node::Free(free_space) = disk_map[left_idx] {
            // Get the last item
            let last_item = disk_map[right_idx];

            match &last_item {
                Node::Free(_) => {
                    // We just ignore it
                    right_idx -= 1;
                }
                Node::File(file) if file.size < free_space => {
                    // We move the whole file, there will still be some free space
                    disk_map.insert(left_idx, Node::File(*file));

                    // Need to bump right_idx back because of the new node
                    right_idx += 1;
                    // We keep track of the free space at the end
                    disk_map[right_idx] = Node::Free(file.size);

                    // We just bump the left idx, the next file will have shifted into the
                    // right_idx
                    left_idx += 1;

                    // Need to update the free space
                    disk_map[left_idx] = Node::Free(free_space - file.size);
                }
                Node::File(file) if file.size == free_space => {
                    // Same as before, except it replaces the free space completely
                    disk_map[left_idx] = Node::File(*file);

                    // And keep track of the empty space
                    disk_map[right_idx] = Node::Free(file.size);

                    // Then we bump both sides
                    left_idx += 1;
                    right_idx -= 1;
                }
                Node::File(file) if split_files => {
                    // We've got to split the file, so we'll replace the free space with a chunk of
                    // the file, leaving the rest at the end for the next loop
                    disk_map[left_idx] = Node::File(File {
                        id: file.id,
                        size: free_space,
                    });

                    disk_map[right_idx] = Node::File(File {
                        id: file.id,
                        size: file.size - free_space,
                    });

                    // We only bump the left idx, as the right idx is pointing at the rest of the
                    // file
                    left_idx += 1;
                }
                Node::File(file) => {
                    // We're not splitting files, so we have to look for a space big enough
                    let mut sub_idx = left_idx;
                    while sub_idx < right_idx {
                        let sub_item = disk_map[sub_idx];
                        match sub_item {
                            Node::File(_) => {
                                sub_idx += 1;
                            }
                            Node::Free(free_space) if free_space == file.size => {
                                // We can fit it here!
                                disk_map[sub_idx] = Node::File(*file);

                                // We have to keep track of the empty space at the end too
                                disk_map[right_idx] = Node::Free(free_space);

                                break; // Done with this file
                            }
                            Node::Free(free_space) if free_space > file.size => {
                                // We move the whole file, there will still be some free space
                                disk_map.insert(sub_idx, Node::File(*file));

                                // Need to bump right_idx back because of the new node
                                right_idx += 1;
                                // We keep track of the free space at the end
                                disk_map[right_idx] = Node::Free(file.size);

                                // We just bump the left idx, the next file will have shifted into the
                                // right_idx
                                sub_idx += 1;

                                // Need to update the free space
                                disk_map[sub_idx] = Node::Free(free_space - file.size);
                                break; // Done with this file
                            }
                            Node::Free(_) => {
                                // Size is too small
                                sub_idx += 1;
                            }
                        }
                    }
                    right_idx -= 1;
                }
            }
        } else {
            panic!("Invalid disk");
        }
    }
}

#[allow(dead_code)]
fn print_disk_map(disk_map: Vec<Node>, left_idx: usize, right_idx: usize) {
    for node in &disk_map {
        match node {
            Node::Free(size) => print!("{}", ".".repeat(*size as usize)),
            Node::File(file) => print!("{}", format!("{}", file.id).repeat(file.size as usize)),
        }
    }
    println!();
    let left_idx_str = " ".repeat(
        disk_map[..left_idx]
            .iter()
            .map(|n| match n {
                Node::Free(size) => *size,
                Node::File(file) => file.size,
            })
            .sum::<u32>() as usize,
    );
    let right_idx_str = " ".repeat(
        disk_map[left_idx..right_idx]
            .iter()
            .map(|n| match n {
                Node::Free(size) => *size,
                Node::File(file) => file.size,
            })
            .sum::<u32>() as usize
            - 1,
    );
    println!("{}L{}R", left_idx_str, right_idx_str);
}

fn calculate_checksum(disk_map: Vec<Node>) -> u64 {
    let mut idx: u64 = 0;
    let mut checksum: u64 = 0;

    for node in disk_map {
        match node {
            Node::Free(size) => {
                // Free space isn't counted, but progresses the idx
                idx += size as u64;
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
    defrag_disk(&mut disk_map, true);

    Ok(calculate_checksum(disk_map))
}

fn part2(input: &str) -> Result<u64> {
    let compressed_disk_map = parse_input(input)?;
    let mut disk_map = expand_disk_map(compressed_disk_map);
    defrag_disk(&mut disk_map, false);

    Ok(calculate_checksum(disk_map))
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 2858);
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
