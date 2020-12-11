from itertools import combinations
from typing import List, Tuple, cast

from advent_of_code import Problem


def get_differences(joltages: List[int]) -> Tuple[int, int, int]:
    return get_differences_from_section([0] + sorted(joltages) + [max(joltages) + 3])


def get_differences_from_section(section: List[int]) -> Tuple[int, int, int]:
    differences: List[int] = [0, 0, 0]

    for idx in range(len(section) - 1):
        differences[section[idx + 1] - section[idx] - 1] += 1

    return cast(Tuple[int, int, int], tuple(differences))


def get_sections(joltages_input: List[int]) -> List[List[int]]:
    # Add the charging outlet
    joltages = [0, *sorted(joltages_input)]
    # Add the device builtin joltage
    joltages.append(joltages[-1] + 3)

    # Sections to look for combinations at
    sections: List[List[int]] = []

    current_section: List[int] = []
    for idx, joltage in enumerate(joltages):
        if not len(current_section):
            if idx:
                if joltage - joltages[idx - 1] == 3:
                    continue
                current_section.append(joltages[idx - 1])

            current_section.append(joltage)
            continue

        difference = joltage - current_section[-1]
        if difference == 3:
            if len(current_section) > 2:
                sections.append(current_section)
            current_section = []
        else:
            current_section.append(joltage)

    return sections


def count_arrangements(joltages_input: List[int]) -> int:
    sections = get_sections(joltages_input)

    arrangements = 1
    for section in sections:
        # Assume each section is short and just brute force each of them
        section_arrangements = 0
        sub_section = section[1:-1]
        for length in range(1, len(sub_section) + 1):
            for combination in combinations(section[1:-1], length):
                try:
                    get_differences_from_section([section[0], *combination, section[-1]])
                except IndexError:
                    # Not valid...
                    continue
                section_arrangements += 1
        try:
            get_differences_from_section([section[0], section[-1]])
        except IndexError:
            pass
        else:
            section_arrangements += 1
        arrangements *= section_arrangements

    return arrangements


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [int(line) for line in self.get_input(__file__)]

        one, _two, three = get_differences(puzzle_input)

        return one * three


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = [int(line) for line in self.get_input(__file__)]

        return count_arrangements(puzzle_input)
