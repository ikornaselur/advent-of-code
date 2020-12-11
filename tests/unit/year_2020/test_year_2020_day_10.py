from advent_of_code.year_2020.day_10 import get_differences


def test_get_differences_basic() -> None:
    joltages = [
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

    one, two, three = get_differences(joltages)

    assert one == 7
    assert two == 0
    assert three == 5


def test_get_differences_bigger_example() -> None:
    joltages = [
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

    one, two, three = get_differences(joltages)

    assert one == 22
    assert two == 0
    assert three == 10
