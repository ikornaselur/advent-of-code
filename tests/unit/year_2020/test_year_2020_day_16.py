from advent_of_code.year_2020.day_16 import (
    parse_rules,
    parse_tickets,
    solve_ticket_map,
)

LINES = [
    "main rule: 10-20 or 30-50",
    "other: 1-3 or 5-7",
    "",
    "your ticket:",
    "10,11,12",
    "",
    "nearby tickets:",
    "14,15,16",
    "17,18,19",
]


def test_parse_rules() -> None:
    assert parse_rules(LINES) == {
        "main rule": (range(10, 21), range(30, 51)),
        "other": (range(1, 4), range(5, 8)),
    }


def test_parse_tickets() -> None:
    assert parse_tickets(LINES, "your ticket:") == [[10, 11, 12]]

    assert parse_tickets(LINES, "nearby tickets:") == [[14, 15, 16], [17, 18, 19]]


def test_solve_ticket_map() -> None:
    rules = {
        "class": (range(0, 2), range(4, 20)),
        "row": (range(0, 6), range(8, 20)),
        "seat": (range(0, 14), range(16, 20)),
    }

    tickets = [
        [11, 12, 13],
        [3, 9, 18],
        [15, 1, 5],
        [5, 14, 9],
    ]

    ticket_map = solve_ticket_map(rules, tickets)

    assert ticket_map == {
        "class": 1,
        "row": 0,
        "seat": 2,
    }


def test_solve_ticket_map_different_input() -> None:
    rules = {
        "a": (range(0, 2), range(4, 10)),
        "b": (range(0, 4), range(2, 10)),
        "c": (range(0, 6), range(9, 10)),
        "d": (range(3, 5), range(7, 10)),
    }

    tickets = [
        [0, 0, 0, 8],
        [1, 1, 3, 8],
        [5, 3, 5, 8],
        [9, 7, 9, 8],
    ]

    ticket_map = solve_ticket_map(rules, tickets)

    assert ticket_map == {
        "a": 0,
        "b": 1,
        "c": 2,
        "d": 3,
    }
