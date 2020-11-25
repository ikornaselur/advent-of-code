from collections import Counter
import os

from advent_of_code import Problem


class Part1(Problem):
    def get_input(self: "Part1") -> str:
        dir_name = os.path.dirname(__file__)
        with open(os.path.join(dir_name, "input.txt"), "r") as f:
            return f.readline()

    def get_solution(self: "Part1") -> int:
        count = Counter(self.get_input())

        return count["("] - count[")"]


class Part2(Part1):
    def get_input(self: "Part1") -> str:
        dir_name = os.path.dirname(__file__)
        with open(os.path.join(dir_name, "input.txt"), "r") as f:
            return f.readline()

    def get_solution(self: "Part1") -> int:
        floor = 0

        puzzle_input = self.get_input()
        for idx, char in enumerate(puzzle_input):
            if char == "(":
                floor += 1
            else:
                floor -= 1

            if floor == -1:
                return idx + 1

        raise Exception("Unable to find instruction taking santa to basement")
