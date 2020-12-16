from advent_of_code.year_2020.day_15 import get_nth_value


def test_get_nth_value() -> None:
    assert get_nth_value([0, 3, 6], 4) == 0
    assert get_nth_value([0, 3, 6], 5) == 3
    assert get_nth_value([0, 3, 6], 6) == 3
    assert get_nth_value([0, 3, 6], 7) == 1
    assert get_nth_value([0, 3, 6], 8) == 0
    assert get_nth_value([0, 3, 6], 9) == 4
    assert get_nth_value([0, 3, 6], 10) == 0
    assert get_nth_value([0, 3, 6], 11) == 2
    assert get_nth_value([0, 3, 6], 12) == 0
    assert get_nth_value([0, 3, 6], 13) == 2
    assert get_nth_value([0, 3, 6], 14) == 2
    assert get_nth_value([0, 3, 6], 2020) == 436

    assert get_nth_value([1, 3, 2], 2020) == 1
    assert get_nth_value([2, 1, 3], 2020) == 10
    assert get_nth_value([1, 2, 3], 2020) == 27
    assert get_nth_value([2, 3, 1], 2020) == 78
    assert get_nth_value([3, 2, 1], 2020) == 438
    assert get_nth_value([3, 1, 2], 2020) == 1836
