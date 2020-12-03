from typing import List, Tuple

from advent_of_code import Problem


def traverse_map(tree_map: List[str], movement: Tuple[int, int]) -> int:
    rows = len(tree_map)
    columns = len(tree_map[0])

    position = [0, 0]

    trees_encountered = 0

    while position[1] < rows:
        if tree_map[position[1]][position[0]] == "#":
            trees_encountered += 1

        position = [(position[0] + movement[0]) % columns, position[1] + movement[1]]

    return trees_encountered


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [row.strip() for row in self.get_input(__file__)]

        return traverse_map(puzzle_input, movement=(3, 1))


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = [row.strip() for row in self.get_input(__file__)]

        return (
            traverse_map(puzzle_input, movement=(1, 1))
            * traverse_map(puzzle_input, movement=(3, 1))
            * traverse_map(puzzle_input, movement=(5, 1))
            * traverse_map(puzzle_input, movement=(7, 1))
            * traverse_map(puzzle_input, movement=(1, 2))
        )

        return len(puzzle_input)
