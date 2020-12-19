from typing import Dict, Tuple, List

from advent_of_code import Problem

X = int
Y = int
Z = int
Coordinate = Tuple[X, Y, Z]
State = bool
World = Dict[Coordinate, State]


def get_active_cubes(cube_coordinate: Coordinate, world: World, max_value: int = 4) -> int:
    count = 0
    for x in range(cube_coordinate[0] - 1, cube_coordinate[0] + 2):
        for y in range(cube_coordinate[1] - 1, cube_coordinate[1] + 2):
            for z in range(cube_coordinate[2] - 1, cube_coordinate[2] + 2):
                current_coordinate = (x, y, z)
                if cube_coordinate == current_coordinate:
                    continue
                if world.get(current_coordinate):
                    count += 1
                if count == max_value:
                    return count
    return count


def get_corners(world: World) -> Tuple[Coordinate, Coordinate]:
    active_coordinates = [coord for coord, state in world.items() if state]

    max_corner: Coordinate = active_coordinates[0]
    min_corner: Coordinate = active_coordinates[0]

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


def tick(world: World) -> World:
    """
    Rules for a tick:
    * If a cube is active and exactly 2 or 3 of its neighbors are also active,
      the cube remains active. Otherwise, the cube becomes inactive.
    * If a cube is inactive but exactly 3 of its neighbors are active, the cube
      becomes active. Otherwise, the cube remains inactive.
    """
    new_world = world.copy()

    # Calculate the edges of the simulation, by finding outermost active cubes:
    min_corner, max_corner = get_corners(world)

    # Fill in empty coordinates as inactive if needed
    for x in range(min_corner[0] - 1, max_corner[0] + 1):
        for y in range(min_corner[1] - 1, max_corner[1] + 1):
            for z in range(min_corner[2] - 1, max_corner[2] + 1):
                inactive_coordinate: Coordinate = (x, y, z)
                if inactive_coordinate not in new_world:
                    new_world[inactive_coordinate] = False

    # Simulate the rules
    for x in range(min_corner[0] - 1, max_corner[0] + 2):
        for y in range(min_corner[1] - 1, max_corner[1] + 2):
            for z in range(min_corner[2] - 1, max_corner[2] + 2):
                current_coordinate: Coordinate = (x, y, z)
                active_cubes = get_active_cubes(current_coordinate, world)
                if world.get(current_coordinate):
                    if active_cubes < 2 or active_cubes > 3:
                        new_world[current_coordinate] = False
                else:
                    if active_cubes == 3:
                        new_world[current_coordinate] = True

    return new_world


def print_world(world: World) -> str:
    min_corner, max_corner = get_corners(world)
    output = []

    for z in range(min_corner[2], max_corner[2] + 1):
        output.append(f"z={z}")
        for x in range(min_corner[0], max_corner[0] + 1):
            row = ""
            for y in range(min_corner[1], max_corner[1] + 1):
                row += "#" if world.get((x, y, z)) else "."
            output.append(row)
        output.append("")

    return "\n".join(output)


def initialise_world(zero_slice: List[str]) -> World:
    world: World = {}
    for x, row in enumerate(zero_slice):
        for y, state in enumerate(row):
            world[x, y, 0] = True if state == "#" else False
    return world


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        world = initialise_world(puzzle_input)

        for _ in range(6):
            world = tick(world)

        return sum(world.values())


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
