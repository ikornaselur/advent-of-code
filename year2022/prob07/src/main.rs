use advent::prelude::*;

mod parse;

#[derive(Debug, PartialEq)]
enum ParsedLine<'a> {
    Cd(&'a str),
    Ls,
    Directory { name: &'a str },
    File { size: usize, name: &'a str },
}

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

#[derive(Debug)]
struct Node {
    path: String,
    children: Vec<Node>,
    files: Vec<File>,
}

impl Node {
    fn new(path: &str) -> Self {
        Node {
            path: path.to_string(),
            children: Vec::new(),
            files: Vec::new(),
        }
    }

    fn file_sizes(&self) -> usize {
        // TODO:: Cache this, as it may get called multiple times depending on file system depth
        // But after solving.. it's so fast that there's no point, but this is ripe for caching
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self.children.iter().map(|c| c.file_sizes()).sum::<usize>()
    }

    fn walk_dirs(&self) -> Vec<&Node> {
        let mut nodes = Vec::new();
        for child in &self.children {
            nodes.push(child);
            nodes.extend(child.walk_dirs());
        }
        nodes
    }
}

fn build_filesystem(node: &mut Node, lines: &[ParsedLine], idx: usize) -> Result<usize> {
    let mut idx = idx;

    loop {
        if idx >= lines.len() {
            break;
        }

        match lines[idx] {
            ParsedLine::Cd("..") => {
                // We exit, as we're leaving this node
                return Ok(idx + 1);
            }
            ParsedLine::Cd(path) => {
                // We navigate to the sub-directory, creating it if it doesnt' exist
                let sub_node = match node.children.iter_mut().find(|n| n.path == path) {
                    Some(n) => n,
                    None => {
                        node.children.push(Node::new(path));
                        node.children.last_mut().unwrap()
                    }
                };

                idx = build_filesystem(sub_node, lines, idx + 1)?;
            }
            ParsedLine::Ls => {
                // We actually don't care about this at the moment, just skip past this line
                idx += 1;
            }
            ParsedLine::File { name, size } => {
                // We add a file to the current node
                node.files.push(File {
                    _name: name.to_string(),
                    size,
                });
                idx += 1;
            }
            ParsedLine::Directory { name } => {
                // We add a directory to the current node
                node.children.push(Node::new(name));
                idx += 1;
            }
        }
    }

    Ok(idx)
}

fn part1(input: &str) -> Result<usize> {
    let parsed_lines = parse::parse_lines(input)?;
    let mut root = Node::new("/");

    // The first line should be cd-ing into "/";
    if let ParsedLine::Cd("/") = parsed_lines[0] {
        build_filesystem(&mut root, &parsed_lines, 1)?;
    } else {
        return Err(error!("First line should be cd-ing into /"));
    }

    // Find all directories that have a file_size of no more than 100_000
    // This includes sub-directories, meaning files can be counted multiple times
    let max_size = 100_000;
    let mut compatible_dirs = Vec::new();
    for dir in root.walk_dirs() {
        if dir.file_sizes() <= max_size {
            compatible_dirs.push(dir)
        }
    }

    Ok(compatible_dirs.iter().map(|d| d.file_sizes()).sum())
}

fn part2(input: &str) -> Result<usize> {
    let total_space = 70_000_000;
    let needed_space = 30_000_000;

    let parsed_lines = parse::parse_lines(input)?;
    let mut root = Node::new("/");

    // The first line should be cd-ing into "/";
    if let ParsedLine::Cd("/") = parsed_lines[0] {
        build_filesystem(&mut root, &parsed_lines, 1)?;
    } else {
        return Err(error!("First line should be cd-ing into /"));
    }

    let root_file_size = root.file_sizes();
    let space_available = total_space - root_file_size;
    let need_to_delete = needed_space - space_available;

    // Now we walk through all dirs until we find the one that's closest to need_to_delete, without
    // going under
    Ok(root.walk_dirs().iter().fold(root_file_size, |acc, &node| {
        let node_file_sizes = node.file_sizes();
        if node_file_sizes > need_to_delete && node_file_sizes < acc {
            node_file_sizes
        } else {
            acc
        }
    }))
    /*
     * An alternative, using filter_map + min, but that does perform potential two iterations over
     * the values, but is it more readable?
    Ok(root
        .walk_dirs()
        .iter()
        .filter_map(|&node| {
            let size = node.file_sizes();
            if size > need_to_delete {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap_or(root_file_size))
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 95_437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 24_933_642);
    }

    #[test]
    fn test_build_filesystem() {
        let mut root = Node::new("/");
        let lines = parse::parse_lines(TEST_INPUT).unwrap();

        build_filesystem(&mut root, &lines, 1).unwrap();

        assert_eq!(root.children.len(), 2);
        assert_eq!(root.files.len(), 2);

        // Let's check a
        let a = root.children.iter().find(|n| n.path == "a").unwrap();
        assert_eq!(a.children.len(), 1);
        assert_eq!(a.files.len(), 3);

        assert_eq!(a.file_sizes(), 94853);
    }
}
