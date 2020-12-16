from typing import Dict, List

from advent_of_code import Problem


def get_nth_value(initial_values: List[int], nth_num: int) -> int:
    values: Dict[int, int] = {}
    idx = 1

    for value in initial_values[:-1]:
        values[value] = idx
        idx += 1

    last_number = initial_values[-1]

    while idx < nth_num:
        if last_number in values:
            next_value = idx - values[last_number]
        else:
            next_value = 0

        values[last_number] = idx
        last_number = next_value

        idx += 1

    return last_number


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)[0]

        return get_nth_value([int(val) for val in puzzle_input.split(",")], 2020)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)[0]

        return get_nth_value([int(val) for val in puzzle_input.split(",")], 30000000)
