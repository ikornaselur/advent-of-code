from dataclasses import dataclass

from advent_of_code import Problem


@dataclass
class ShipPosition:
    x: int
    y: int
    direction: int  # 0 = North, 1 = East, 2 = South, 3 = West

    def move(self: "ShipPosition", instruction: str) -> None:
        action = instruction[:1]
        amount = int(instruction[1:])

        if action == "N":
            self.x += amount
        elif action == "E":
            self.y += amount
        elif action == "S":
            self.x -= amount
        elif action == "W":
            self.y -= amount
        elif action == "L":
            self.direction = (self.direction - amount // 90) % 4
        elif action == "R":
            self.direction = (self.direction + amount // 90) % 4
        elif action == "F":
            if self.direction == 0:
                self.x += amount
            elif self.direction == 1:
                self.y += amount
            elif self.direction == 2:
                self.x -= amount
            elif self.direction == 3:
                self.y -= amount
        else:
            raise ValueError(f"Unknown action: {action}")

    def __repr__(self: "ShipPosition") -> str:
        facing = ["North", "East", "South", "West"]

        return f"<ShipPosition: ({self.x},{self.y}) - facing {facing[self.direction]}"


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        position = ShipPosition(0, 0, 1)

        for instruction in puzzle_input:
            position.move(instruction)

        return abs(position.x) + abs(position.y)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
