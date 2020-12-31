import re
from typing import List, Set, Tuple

from advent_of_code import Problem


class Tile:
    _tile: List[str]
    id_num: int
    sides: Set[str]

    def __init__(self: "Tile", tile: List[str]) -> None:
        self._tile = tile[1:]
        self.id_num = int(re.findall(r"\d+", tile[0])[0])

        # Add all sides
        self.sides = {
            self._tile[0],  # Top
            self._tile[-1],  # Bottom
            "".join(row[0] for row in self._tile),  # Left
            "".join(row[-1] for row in self._tile),  # Right
        }

        # Add the reversed of the sides as well
        self.sides.update([side[::-1] for side in self.sides])

    def adjacent(self: "Tile", other: "Tile") -> bool:
        return len(self.sides.intersection(other.sides)) > 0

    def __repr__(self: "Tile") -> str:
        return f"<Tile {self.id_num}>"


def find_corners(tiles: List[Tile]) -> Tuple[Tile, Tile, Tile, Tile]:
    corners = []
    for tile in tiles:
        adjacent = 0
        for other in tiles:
            if tile == other:
                continue
            if tile.adjacent(other):
                adjacent += 1
            if adjacent > 2:
                break
        if adjacent == 2:
            corners.append(tile)

    assert len(corners) == 4, f"Length is {len(corners)}"
    return corners[0], corners[1], corners[2], corners[3]


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        # Tiles are square, so find width of first tile and chunk input with the width + 2 (header and space)
        height = len(puzzle_input[1]) + 2

        # Remove 1 from the height to ignore the spacing between the tiles
        tiles = [
            Tile(puzzle_input[i : i + height - 1]) for i in range(0, len(puzzle_input), height)
        ]

        a, b, c, d = find_corners(tiles)

        return a.id_num * b.id_num * c.id_num * d.id_num


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
