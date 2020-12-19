from typing import Dict, List, Tuple

from advent_of_code import Problem

X = int
Y = int
Z = int
W = int
Coordinate3D = Tuple[X, Y, Z]
Coordinate4D = Tuple[X, Y, Z, W]
State = bool
World3D = Dict[Coordinate3D, State]
World4D = Dict[Coordinate4D, State]


class Simulation3D:
    ticks: int
    world: World3D

    def __init__(self: "Simulation3D", initial_slice: List[str]) -> None:
        self.ticks = 0
        self.world: World3D = {}

        for x, row in enumerate(initial_slice):
            for y, state in enumerate(row):
                self.world[x, y, 0] = True if state == "#" else False

    def get_active_cubes(
        self: "Simulation3D", cube_coordinate: Coordinate3D, max_value: int = 4
    ) -> int:
        count = 0
        for x in range(cube_coordinate[0] - 1, cube_coordinate[0] + 2):
            for y in range(cube_coordinate[1] - 1, cube_coordinate[1] + 2):
                for z in range(cube_coordinate[2] - 1, cube_coordinate[2] + 2):
                    current_coordinate = (x, y, z)
                    if cube_coordinate == current_coordinate:
                        continue
                    if self.world.get(current_coordinate):
                        count += 1
                    if count == max_value:
                        return count
        return count

    def get_corners(self: "Simulation3D") -> Tuple[Coordinate3D, Coordinate3D]:
        active_coordinates = [coord for coord, state in self.world.items() if state]

        max_corner: Coordinate3D = active_coordinates[0]
        min_corner: Coordinate3D = active_coordinates[0]

        for coordinate in active_coordinates:
            max_corner = (
                max(max_corner[0], coordinate[0]),
                max(max_corner[1], coordinate[1]),
                max(max_corner[2], coordinate[2]),
            )
            min_corner = (
                min(min_corner[0], coordinate[0]),
                min(min_corner[1], coordinate[1]),
                min(min_corner[2], coordinate[2]),
            )

        return min_corner, max_corner

    def tick(self: "Simulation3D") -> None:
        """
        Rules for a tick:
        * If a cube is active and exactly 2 or 3 of its neighbors are also active,
          the cube remains active. Otherwise, the cube becomes inactive.
        * If a cube is inactive but exactly 3 of its neighbors are active, the cube
          becomes active. Otherwise, the cube remains inactive.
        """
        new_world = self.world.copy()

        # Calculate the edges of the simulation, by finding outermost active cubes:
        min_corner, max_corner = self.get_corners()

        # Fill in empty coordinates as inactive if needed
        for x in range(min_corner[0] - 1, max_corner[0] + 1):
            for y in range(min_corner[1] - 1, max_corner[1] + 1):
                for z in range(min_corner[2] - 1, max_corner[2] + 1):
                    inactive_coordinate: Coordinate3D = (x, y, z)
                    if inactive_coordinate not in new_world:
                        new_world[inactive_coordinate] = False

        # Simulate the rules
        for x in range(min_corner[0] - 1, max_corner[0] + 2):
            for y in range(min_corner[1] - 1, max_corner[1] + 2):
                for z in range(min_corner[2] - 1, max_corner[2] + 2):
                    current_coordinate: Coordinate3D = (x, y, z)
                    active_cubes = self.get_active_cubes(current_coordinate)
                    if self.world.get(current_coordinate):
                        if active_cubes < 2 or active_cubes > 3:
                            new_world[current_coordinate] = False
                    else:
                        if active_cubes == 3:
                            new_world[current_coordinate] = True

        self.world = new_world

    def print_sim(self: "Simulation3D") -> str:
        min_corner, max_corner = self.get_corners()
        output = []

        for z in range(min_corner[2], max_corner[2] + 1):
            output.append(f"z={z}")
            for x in range(min_corner[0], max_corner[0] + 1):
                row = ""
                for y in range(min_corner[1], max_corner[1] + 1):
                    row += "#" if self.world.get((x, y, z)) else "."
                output.append(row)
            output.append("")

        return "\n".join(output)


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        simulation = Simulation3D(puzzle_input)

        for _ in range(6):
            simulation.tick()

        return sum(simulation.world.values())


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
