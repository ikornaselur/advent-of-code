from advent_of_code.year_2020.day_09 import (
    get_contiguous_set,
    get_first_invalid,
)


def test_get_first_invalid() -> None:
    feed = [
        35,
        20,
        15,
        25,
        47,
        40,
        62,
        55,
        65,
        95,
        102,
        117,
        150,
        182,
        127,
        219,
        299,
        277,
        309,
        576,
    ]

    assert get_first_invalid(feed, preamble=5) == 127


def test_get_contiguous_set() -> None:
    feed = [
        35,
        20,
        15,
        25,
        47,
        40,
        62,
        55,
        65,
        95,
        102,
        117,
        150,
        182,
        127,
        219,
        299,
        277,
        309,
        576,
    ]

    assert get_contiguous_set(127, feed) == [15, 25, 47, 40]
