import re
from enum import Enum, auto
from typing import Tuple

from advent_of_code import Problem


class Action(Enum):
    on = auto()
    off = auto()
    toggle = auto()


Coordinate = Tuple[int, int]
Instruction = Tuple[Action, Coordinate, Coordinate]


class Part1(Problem):
    def __init__(self: "Part1", width: int = 1000, height: int = 1000) -> None:
        super().__init__()

        self._lights = [[0 for _ in range(width)] for _ in range(height)]

    def parse_instruction(self: "Part1", instruction: str) -> Instruction:
        pattern = re.compile(
            r"(?P<action>(turn o(n|ff)|toggle)) "
            r"(?P<start_x>\d+),(?P<start_y>\d+) "
            r"through (?P<end_x>\d+),(?P<end_y>\d+)"
        )
        match = pattern.search(instruction)
        if match is None:
            raise Exception("Invalid instruction")
        raw_action = match.group("action")
        if raw_action == "turn on":
            action = Action.on
        elif raw_action == "turn off":
            action = Action.off
        elif raw_action == "toggle":
            action = Action.toggle
        else:
            raise Exception("Unknown action")

        start = (int(match.group("start_x")), int(match.group("start_y")))
        end = (int(match.group("end_x")), int(match.group("end_y")))

        return (action, start, end)

    def execute_instruction(self: "Part1", instruction: Instruction) -> None:
        action, start, end = instruction
        if action == Action.on:
            for x in range(start[0], end[0] + 1):
                for y in range(start[1], end[1] + 1):
                    self._lights[x][y] = 1
        elif action == Action.off:
            for x in range(start[0], end[0] + 1):
                for y in range(start[1], end[1] + 1):
                    self._lights[x][y] = 0
        elif action == Action.toggle:
            for x in range(start[0], end[0] + 1):
                for y in range(start[1], end[1] + 1):
                    self._lights[x][y] = abs(self._lights[x][y] - 1)

    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        for line in puzzle_input:
            instruction = self.parse_instruction(line)
            self.execute_instruction(instruction)

        return sum(len([x for x in row if x]) for row in self._lights)


class Part2(Part1):
    def execute_instruction(self: "Part2", instruction: Instruction) -> None:
        action, start, end = instruction
        if action == Action.on:
            for x in range(start[0], end[0] + 1):
                for y in range(start[1], end[1] + 1):
                    self._lights[x][y] += 1
        elif action == Action.off:
            for x in range(start[0], end[0] + 1):
                for y in range(start[1], end[1] + 1):
                    self._lights[x][y] = max(0, self._lights[x][y] - 1)
        elif action == Action.toggle:
            for x in range(start[0], end[0] + 1):
                for y in range(start[1], end[1] + 1):
                    self._lights[x][y] += 2

    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        for line in puzzle_input:
            instruction = self.parse_instruction(line)
            self.execute_instruction(instruction)

        return sum(sum(row) for row in self._lights)
