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


@dataclass
class WaypointPosition:
    ship_x: int
    ship_y: int
    waypoint_x: int
    waypoint_y: int

    def move(self: "WaypointPosition", instruction: str) -> None:
        action = instruction[:1]
        amount = int(instruction[1:])

        if action == "N":
            self.waypoint_x += amount
        elif action == "E":
            self.waypoint_y += amount
        elif action == "S":
            self.waypoint_x -= amount
        elif action == "W":
            self.waypoint_y -= amount
        elif action == "L":
            if amount // 90 % 4 == 1:
                self.waypoint_x, self.waypoint_y = self.waypoint_y, self.waypoint_x * -1
            elif amount // 90 % 4 == 2:
                self.waypoint_x *= -1
                self.waypoint_y *= -1
            elif amount // 90 % 4 == 3:
                self.waypoint_x, self.waypoint_y = self.waypoint_y * -1, self.waypoint_x
            else:
                raise ValueError(f"Invalid amount for rotation: {amount}")
        elif action == "R":
            if amount // 90 % 4 == 1:
                self.waypoint_x, self.waypoint_y = self.waypoint_y * -1, self.waypoint_x
            elif amount // 90 % 4 == 2:
                self.waypoint_x *= -1
                self.waypoint_y *= -1
            elif amount // 90 % 4 == 3:
                self.waypoint_x, self.waypoint_y = self.waypoint_y, self.waypoint_x * -1
            else:
                raise ValueError(f"Invalid amount for rotation: {amount}")
        elif action == "F":
            self.ship_x += self.waypoint_x * amount
            self.ship_y += self.waypoint_y * amount
        else:
            raise ValueError(f"Unknown action: {action}")

    def __repr__(self: "WaypointPosition") -> str:
        return (
            f"<WaypointPosition: Ship ({self.ship_x},{self.ship_y}) "
            f"Waypoint ({self.waypoint_x},{self.waypoint_y})"
        )


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

        position = WaypointPosition(0, 0, 1, 10)

        for instruction in puzzle_input:
            position.move(instruction)

        return abs(position.ship_x) + abs(position.ship_y)
