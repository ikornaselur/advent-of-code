from advent_of_code.year_2020.day_19 import count_valid_strings, parse_rule


def test_parse_rules() -> None:
    rules = [
        "0: 1 2",
        '1: "a"',
        "2: 1 3 | 3 1",
        '3: "b"',
    ]

    assert parse_rule(rules, 0) == "a(ab|ba)"
    assert parse_rule(rules, 1) == "a"
    assert parse_rule(rules, 2) == "(ab|ba)"
    assert parse_rule(rules, 3) == "b"


def test_count_valid_strings() -> None:
    rules = [
        "0: 4 1 5",
        "1: 2 3 | 3 2",
        "2: 4 4 | 5 5",
        "3: 4 5 | 5 4",
        '4: "a"',
        '5: "b"',
    ]
    rule = parse_rule(rules, 0)

    strings = [
        "ababbb",
        "bababa",
        "abbbab",
        "aaabbb",
        "aaaabbb",
    ]

    assert count_valid_strings(strings, rule) == 2
