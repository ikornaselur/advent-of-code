from typing import List, Tuple, cast

from advent_of_code import Problem


def get_differences(joltages: List[int]) -> Tuple[int, int, int]:
    # Add the charging outlet
    joltages.append(0)
    joltages.sort()
    # Add the device builtin joltage
    joltages.append(joltages[-1] + 3)

    differences: List[int] = [0, 0, 0]

    for idx in range(len(joltages) - 1):
        differences[joltages[idx + 1] - joltages[idx] - 1] += 1

    return cast(Tuple[int, int, int], tuple(differences))


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [int(line) for line in self.get_input(__file__)]

        one, two, three = get_differences(puzzle_input)

        return one * three


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
