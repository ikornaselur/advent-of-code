from typing import List

from advent_of_code import Problem


def more_than_occupied(
    seat_map: List[str],
    height: int,
    width: int,
    only_close: bool,
    seat_x: int,
    seat_y: int,
    max_occupied: int,
) -> bool:
    count = 0
    for x in range(max(0, seat_x - 1), min(seat_x + 2, height)):
        for y in range(max(0, seat_y - 1), min(seat_y + 2, width)):
            if x == seat_x and y == seat_y:
                # only check around
                continue

            if only_close:
                # Just checking the immediate seat
                if seat_map[x][y] == "#":
                    count += 1
                    if count > max_occupied:
                        return True
            else:
                # Check until we reach a seat or an edge
                direction = (x - seat_x, y - seat_y)
                current_location = (x, y)
                while True:
                    if not (0 <= current_location[0] < height) or not (
                        0 <= current_location[1] < width
                    ):
                        break
                    seat_check = seat_map[current_location[0]][current_location[1]]
                    if seat_check == ".":
                        current_location = (
                            current_location[0] + direction[0],
                            current_location[1] + direction[1],
                        )
                        continue
                    elif seat_check == "L":
                        break
                    elif seat_check == "#":
                        count += 1
                        if count > max_occupied:
                            return True
                        break

    return False


def tick(seat_map: List[str], only_close: bool = True, tolerance: int = 3) -> List[str]:
    height = len(seat_map)
    width = len(seat_map[0])

    def foo_bar(seat_x: int, seat_y: int, max_occupied: int) -> bool:
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

    def _more_than_occupied(x: int, y: int, number: int) -> bool:
        if only_close:
            return foo_bar(x, y, number)
        return more_than_occupied(seat_map, height, width, only_close, x, y, number)

    new_map: List[str] = []
    for x, row in enumerate(seat_map):
        new_row = []
        for y, seat in enumerate(row):
            if seat == "L" and not _more_than_occupied(x, y, 0):
                new_row.append("#")
            elif seat == "#" and _more_than_occupied(x, y, tolerance):
                new_row.append("L")
            else:
                new_row.append(seat)

        new_map.append("".join(new_row))

    return new_map


def stable_seats(seat_map: List[str], only_close: bool = True, tolerance: int = 3) -> int:
    while seat_map != (seat_map := tick(seat_map, only_close, tolerance)):
        pass
    return len([char for row in seat_map for char in row if char == "#"])


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [row.strip() for row in self.get_input(__file__)]

        return stable_seats(puzzle_input)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = [row.strip() for row in self.get_input(__file__)]

        return stable_seats(puzzle_input, only_close=False, tolerance=4)
