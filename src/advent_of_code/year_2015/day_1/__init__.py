from collections import Counter

from advent_of_code import Problem


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        count = Counter(self.get_input(__file__)[0])

        return count["("] - count[")"]


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        floor = 0

        puzzle_input = self.get_input(__file__)[0]
        for idx, char in enumerate(puzzle_input):
            if char == "(":
                floor += 1
            else:
                floor -= 1

            if floor == -1:
                return idx + 1

        raise Exception("Unable to find instruction taking santa to basement")
