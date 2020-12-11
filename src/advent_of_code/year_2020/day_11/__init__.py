from typing import List

from advent_of_code import Problem


def tick(seat_map: List[str]) -> List[str]:
    height = len(seat_map)
    width = len(seat_map[0])

    def more_than_occupied(seat_x: int, seat_y: int, max_occupied: int) -> bool:
        count = 0
        for x in range(max(0, seat_x - 1), min(seat_x + 2, height)):
            for y in range(max(0, seat_y - 1), min(seat_y + 2, width)):
                if x == seat_x and y == seat_y:
                    # only check around
                    continue

                if seat_map[x][y] == "#":
                    count += 1
                    if count > max_occupied:
                        return True
        return False

    new_map: List[str] = []
    for x, row in enumerate(seat_map):
        new_row = []
        for y, seat in enumerate(row):
            if seat == "L" and not more_than_occupied(x, y, 0):
                new_row.append("#")
            elif seat == "#" and more_than_occupied(x, y, 3):
                new_row.append("L")
            else:
                new_row.append(seat)

        new_map.append("".join(new_row))

    return new_map


def stable_seats(seat_map: List[str]) -> int:
    while seat_map != (seat_map := tick(seat_map)):
        pass
    return len([char for row in seat_map for char in row if char == "#"])


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [row.strip() for row in self.get_input(__file__)]

        return stable_seats(puzzle_input)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
