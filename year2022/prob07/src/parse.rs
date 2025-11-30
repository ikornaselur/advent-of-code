use advent::parsers::nom_unsigned_digit;
use advent::prelude::*;

use crate::ParsedLine;

fn nom_cd(input: &str) -> IResult<&str, ParsedLine<'_>> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, path) = not_line_ending(input)?;

    Ok((input, ParsedLine::Cd(path)))
}

fn nom_ls(input: &str) -> IResult<&str, ParsedLine<'_>> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, ParsedLine::Ls))
}

fn nom_directory(input: &str) -> IResult<&str, ParsedLine<'_>> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = not_line_ending(input)?;
    Ok((input, ParsedLine::Directory { name }))
}

fn nom_file(input: &str) -> IResult<&str, ParsedLine<'_>> {
    let (input, (size, name)) =
        separated_pair(nom_unsigned_digit::<usize>, space1, not_line_ending)(input)?;

    Ok((input, ParsedLine::File { size, name }))
}

fn nom_output(input: &str) -> IResult<&str, ParsedLine<'_>> {
    alt((nom_directory, nom_file))(input)
}

fn nom_line(input: &str) -> IResult<&str, ParsedLine<'_>> {
    alt((nom_cd, nom_ls, nom_output))(input)
}

pub fn parse_lines(input: &str) -> Result<Vec<ParsedLine<'_>>> {
    let (_, parsed_lines) = separated_list0(line_ending, nom_line)(input)
        .map_err(|e| error!("Unable to parse: {}", e))?;

    Ok(parsed_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_nom_cd() {
        let (input, parsed) = nom_cd("$ cd /home/user\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(parsed, ParsedLine::Cd("/home/user"));
    }

    #[test]
    fn test_nom_ls() {
        let (input, parsed) = nom_ls("$ ls\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(parsed, ParsedLine::Ls);
    }

    #[test]
    fn test_nom_output() {
        let (input, parsed) = nom_output("dir foo\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(parsed, ParsedLine::Directory { name: "foo" });
    }

    #[test]
    fn test_nom_line() {
        let (input, parsed) = nom_line("$ cd /home/user\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(parsed, ParsedLine::Cd("/home/user"));

        let (input, parsed) = nom_line("$ ls\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(parsed, ParsedLine::Ls);

        let (input, parsed) = nom_line("dir foo\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(parsed, ParsedLine::Directory { name: "foo" });
    }

    #[test]
    fn test_parse_lines() {
        let parsed_lines = parse_lines(TEST_INPUT).unwrap();

        assert_eq!(
            parsed_lines,
            vec![
                ParsedLine::Cd("/"),
                ParsedLine::Ls,
                ParsedLine::Directory { name: "a" },
                ParsedLine::File {
                    size: 14848514,
                    name: "b.txt"
                },
                ParsedLine::File {
                    size: 8504156,
                    name: "c.dat"
                },
                ParsedLine::Directory { name: "d" },
                ParsedLine::Cd("a"),
                ParsedLine::Ls,
                ParsedLine::Directory { name: "e" },
                ParsedLine::File {
                    size: 29116,
                    name: "f"
                },
                ParsedLine::File {
                    size: 2557,
                    name: "g"
                },
                ParsedLine::File {
                    size: 62596,
                    name: "h.lst"
                },
                ParsedLine::Cd("e"),
                ParsedLine::Ls,
                ParsedLine::File {
                    size: 584,
                    name: "i"
                },
                ParsedLine::Cd(".."),
                ParsedLine::Cd(".."),
                ParsedLine::Cd("d"),
                ParsedLine::Ls,
                ParsedLine::File {
                    size: 4060174,
                    name: "j"
                },
                ParsedLine::File {
                    size: 8033020,
                    name: "d.log"
                },
                ParsedLine::File {
                    size: 5626152,
                    name: "d.ext"
                },
                ParsedLine::File {
                    size: 7214296,
                    name: "k"
                },
            ]
        );
    }
}
