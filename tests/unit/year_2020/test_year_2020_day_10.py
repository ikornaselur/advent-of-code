from advent_of_code.year_2020.day_10 import count_arrangements, get_differences


def test_get_differences_basic_example() -> None:
    one, two, three = get_differences(BASIC_JOLTAGES)

    assert one == 7
    assert two == 0
    assert three == 5


def test_get_differences_bigger_example() -> None:
    one, two, three = get_differences(BIGGER_JOLTAGES)

    assert one == 22
    assert two == 0
    assert three == 10


def test_count_arrangements_basic_example() -> None:
    assert count_arrangements(BASIC_JOLTAGES) == 8


def test_count_arrangements_bigger_example() -> None:
    assert count_arrangements(BIGGER_JOLTAGES) == 19208


BASIC_JOLTAGES = [
    16,
    10,
    15,
    5,
    1,
    11,
    7,
    19,
    6,
    12,
    4,
]

BIGGER_JOLTAGES = [
    28,
    33,
    18,
    42,
    31,
    14,
    46,
    20,
    48,
    47,
    24,
    23,
    49,
    45,
    19,
    38,
    39,
    11,
    1,
    32,
    25,
    35,
    8,
    17,
    7,
    9,
    4,
    2,
    34,
    10,
    3,
]
