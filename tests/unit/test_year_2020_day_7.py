from advent_of_code.year_2020.day_7 import (
    bag_levels,
    can_contain,
    parse_rule,
    parse_rule_level,
)


def test_parse_rule() -> None:
    assert parse_rule("faded blue bags contain no other bags.") == (
        "faded blue",
        [],
    )
    assert parse_rule("bright white bags contain 1 shiny gold bag.") == (
        "bright white",
        ["shiny gold"],
    )
    assert parse_rule("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.") == (
        "vibrant plum",
        ["faded blue", "dotted black"],
    )


def test_can_contan() -> None:
    rules = list(
        map(
            parse_rule,
            [
                "light red bags contain 1 bright white bag, 2 muted yellow bags.",
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                "bright white bags contain 1 shiny gold bag.",
                "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                "faded blue bags contain no other bags.",
                "dotted black bags contain no other bags.",
            ],
        )
    )

    assert can_contain("shiny gold", rules) == 4


def test_parse_rule_level() -> None:
    assert parse_rule_level("faded blue bags contain no other bags.") == (
        "faded blue",
        [],
    )
    assert parse_rule_level("bright white bags contain 1 shiny gold bag.") == (
        "bright white",
        [(1, "shiny gold")],
    )
    assert parse_rule_level(
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."
    ) == (
        "vibrant plum",
        [(5, "faded blue"), (6, "dotted black")],
    )


def test_bag_levels() -> None:
    rules = list(
        map(
            parse_rule_level,
            [
                "light red bags contain 1 bright white bag, 2 muted yellow bags.",
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                "bright white bags contain 1 shiny gold bag.",
                "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                "faded blue bags contain no other bags.",
                "dotted black bags contain no other bags.",
            ],
        )
    )

    assert bag_levels("faded blue", rules) == 0
    assert bag_levels("dark olive", rules) == 7
    assert bag_levels("vibrant plum", rules) == 11
    assert bag_levels("shiny gold", rules) == 32
