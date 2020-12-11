from advent_of_code.year_2020.day_06 import count_all_answered_yes, count_yesses


def test_count_yesses() -> None:
    test_input = [
        "abc",
        "",
        "a",
        "b",
        "c",
        "",
        "ab",
        "ac",
        "",
        "a",
        "a",
        "a",
        "a",
        "",
        "b",
    ]

    assert count_yesses(test_input) == 11


def test_count_all_answered_yes() -> None:
    test_input = [
        "abc",
        "",
        "a",
        "b",
        "c",
        "",
        "ab",
        "ac",
        "",
        "a",
        "a",
        "a",
        "a",
        "",
        "b",
    ]

    assert count_all_answered_yes(test_input) == 6
