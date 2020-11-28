from advent_of_code import Problem


def total_chars(string: str) -> int:
    return len(string)


def total_memory(string: str) -> int:
    return len(string.encode("utf-8").decode("unicode-escape")) - 2


def encode(string: str) -> str:
    # Encoding suitable as the contents of a Unicode literal in ASCII-encoded
    # Python source code, except that quotes are not escaped
    # See: https://docs.python.org/3/library/codecs.html#text-encodings
    encoded = string.encode("unicode-escape").decode("utf-8").replace('"', r"\"")
    return f'"{encoded}"'


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        diff = 0

        for line in puzzle_input:
            diff += total_chars(line.strip()) - total_memory(line.strip())

        return diff


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        diff = 0

        for line in puzzle_input:
            diff += total_chars(encode(line.strip())) - total_chars(line.strip())

        return diff
