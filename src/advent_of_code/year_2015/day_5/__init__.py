import re

from advent_of_code import Problem

FORBIDDEN_TUPLES = ("ab", "cd", "pq", "xy")


class Part1(Problem):
    def _has_three_vowels(self: "Part1", string: str) -> bool:
        return len(re.sub(r"[^aeiou]", "", string)) >= 3

    def _has_repeated_letters(self: "Part1", string: str) -> bool:
        if len(string) < 2:
            return False

        prev = string[0]
        for char in string[1:]:
            if char == prev:
                return True
            prev = char

        return False

    def _has_forbidden_tuples(self: "Part1", string: str) -> bool:
        if len(string) < 2:
            return False

        for tup in FORBIDDEN_TUPLES:
            if tup in string:
                return True

        return False

    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        count = 0

        for line in puzzle_input:
            if not self._has_three_vowels(line):
                continue
            if not self._has_repeated_letters(line):
                continue
            if self._has_forbidden_tuples(line):
                continue

            count += 1

        return count


class Part2(Problem):
    def _has_non_overlapping_pair(self: "Part2", string: str) -> bool:
        if len(string) < 4:
            return False
        pairs = {string[0:2]}
        last_pair = string[1:3]

        # idx is upper limit
        idx = 4
        while idx <= len(string):
            next_pair = string[idx - 2 : idx]
            if next_pair in pairs:
                return True

            pairs.add(last_pair)
            last_pair = next_pair
            idx += 1

        return False

    def _has_repeat_one_letter_apart(self: "Part2", string: str) -> bool:
        if len(string) < 3:
            return False

        # idx is upper limit
        idx = 2
        while idx < len(string):
            if string[idx - 2] == string[idx]:
                return True
            idx += 1

        return False

    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        count = 0

        for line in puzzle_input:
            if not self._has_non_overlapping_pair(line):
                continue
            if not self._has_repeat_one_letter_apart(line):
                continue

            count += 1

        return count
