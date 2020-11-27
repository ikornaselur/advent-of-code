from typing import Tuple

from advent_of_code import Problem


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)[0]

        location = (0, 0)
        visited = {location}

        for instruction in puzzle_input:
            if instruction == "^":
                location = (location[0] + 1, location[1])
            elif instruction == ">":
                location = (location[0], location[1] + 1)
            elif instruction == "v":
                location = (location[0] - 1, location[1])
            elif instruction == "<":
                location = (location[0], location[1] - 1)

            if location not in visited:
                visited.add(location)

        return len(visited)


class Part2(Problem):
    def update_location(
        self: "Part2", instruction: str, location: Tuple[int, int]
    ) -> Tuple[int, int]:
        if instruction == "^":
            return (location[0] + 1, location[1])
        elif instruction == ">":
            return (location[0], location[1] + 1)
        elif instruction == "v":
            return (location[0] - 1, location[1])
        else:
            return (location[0], location[1] - 1)

    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)[0]

        santa_location = (0, 0)
        robo_location = (0, 0)
        visited = {santa_location}

        for idx, instruction in enumerate(puzzle_input):
            if idx % 2 == 0:
                santa_location = self.update_location(instruction, santa_location)
                if santa_location not in visited:
                    visited.add(santa_location)
            else:
                robo_location = self.update_location(instruction, robo_location)
                if robo_location not in visited:
                    visited.add(robo_location)

        return len(visited)
