from advent_of_code.year_2020.day_11 import (
    more_than_occupied,
    stable_seats,
    tick,
)


def test_tick_basic() -> None:
    seat_map = [
        "#.##",
        "####",
        "#.#.",
        "####",
    ]

    assert tick(seat_map) == [
        "#.L#",
        "#LLL",
        "L.L.",
        "#L##",
    ]


def test_tick() -> None:
    seat_map = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ]

    seat_map = tick(seat_map)

    assert seat_map == [
        "#.##.##.##",
        "#######.##",
        "#.#.#..#..",
        "####.##.##",
        "#.##.##.##",
        "#.#####.##",
        "..#.#.....",
        "##########",
        "#.######.#",
        "#.#####.##",
    ]

    seat_map = tick(seat_map)

    assert seat_map == [
        "#.LL.L#.##",
        "#LLLLLL.L#",
        "L.L.L..L..",
        "#LLL.LL.L#",
        "#.LL.LL.LL",
        "#.LLLL#.##",
        "..L.L.....",
        "#LLLLLLLL#",
        "#.LLLLLL.L",
        "#.#LLLL.##",
    ]


def test_stable_tick() -> None:
    seat_map = [
        "#.#L.L#.##",
        "#LLL#LL.L#",
        "L.#.L..#..",
        "#L##.##.L#",
        "#.#L.LL.LL",
        "#.#L#L#.##",
        "..L.L.....",
        "#L#L##L#L#",
        "#.LLLLLL.L",
        "#.#L#L#.##",
    ]

    assert tick(seat_map) == seat_map


def test_stable_seats() -> None:
    seat_map = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ]

    assert stable_seats(seat_map) == 37


def test_stable_seats_distant() -> None:
    seat_map = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ]

    assert stable_seats(seat_map, only_close=False, tolerance=4) == 26


def test_more_than_occupied_distant() -> None:
    seat_map = [
        ".......#.",
        "...#.....",
        ".#.......",
        ".........",
        "..#L....#",
        "....#....",
        ".........",
        "#........",
        "...#.....",
    ]
    # Test close first by testing max at 2 and 1
    assert (
        more_than_occupied(seat_map, 9, 9, only_close=True, seat_x=4, seat_y=3, max_occupied=2)
        is False
    )
    assert (
        more_than_occupied(seat_map, 9, 9, only_close=True, seat_x=4, seat_y=3, max_occupied=1)
        is True
    )

    # Check that we find 8 by testing max at 8 and 7
    assert (
        more_than_occupied(seat_map, 9, 9, only_close=False, seat_x=4, seat_y=3, max_occupied=8)
        is False
    )
    assert (
        more_than_occupied(seat_map, 9, 9, only_close=False, seat_x=4, seat_y=3, max_occupied=7)
        is True
    )
