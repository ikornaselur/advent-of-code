import re

from advent_of_code import Problem

PATTERN = re.compile(
    r"(?P<min_count>\d+)-(?P<max_count>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)"
)


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        valid_count = 0

        for string in puzzle_input:
            match = PATTERN.search(string)
            if match is None:
                raise Exception("Regex failed")

            min_count = int(match.group("min_count"))
            max_count = int(match.group("max_count"))
            letter = match.group("letter")
            password = match.group("password")

            if min_count <= password.count(letter) <= max_count:
                valid_count += 1

        return valid_count


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        valid_count = 0

        for string in puzzle_input:
            match = PATTERN.search(string)
            if match is None:
                raise Exception("Regex failed")

            # Idx starts at 1, not 0, in the input
            first_idx = int(match.group("min_count")) - 1
            second_idx = int(match.group("max_count")) - 1

            letter = match.group("letter")
            password = match.group("password")

            if (password[first_idx] == letter) != (password[second_idx] == letter):
                valid_count += 1

        return valid_count
