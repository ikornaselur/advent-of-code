from typing import Tuple

from advent_of_code import Problem


def decode_seat(seat: str) -> Tuple[int, int]:
    seat_bin = seat.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1")

    row = int(seat_bin[:-3], 2)
    column = int(seat_bin[-3:], 2)

    return row, column


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        max_id = 0
        for seat in puzzle_input:
            row, column = decode_seat(seat.strip())

            if (seat_id := row * 8 + column) > max_id:
                max_id = seat_id

        return max_id


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        seat_ids = set()

        for seat in puzzle_input:
            row, column = decode_seat(seat.strip())
            seat_ids.add(row * 8 + column)

        min_id = min(seat_ids)
        max_id = max(seat_ids)

        for seat_id in range(min_id + 1, max_id - 1):
            if seat_id not in seat_ids:
                return seat_id

        raise Exception("Seat not found!")
