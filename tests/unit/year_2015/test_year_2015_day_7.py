from advent_of_code.year_2015.day_7 import (
    Gate,
    Instruction,
    parse_instruction,
    solve,
)


def test_parse_instruction() -> None:
    assert parse_instruction("123 -> x") == Instruction(
        sources=[123],
        destination="x",
        gate=Gate.NONE,
        original="123 -> x",
    )

    assert parse_instruction("456 -> y") == Instruction(
        sources=[456],
        destination="y",
        gate=Gate.NONE,
        original="456 -> y",
    )

    assert parse_instruction("x AND y -> d") == Instruction(
        sources=["x", "y"],
        destination="d",
        gate=Gate.AND,
        original="x AND y -> d",
    )

    assert parse_instruction("x OR y -> e") == Instruction(
        sources=["x", "y"],
        destination="e",
        gate=Gate.OR,
        original="x OR y -> e",
    )

    assert parse_instruction("x LSHIFT 2 -> f") == Instruction(
        sources=["x", 2],
        destination="f",
        gate=Gate.LSHIFT,
        original="x LSHIFT 2 -> f",
    )

    assert parse_instruction("y RSHIFT 2 -> g") == Instruction(
        sources=["y", 2],
        destination="g",
        gate=Gate.RSHIFT,
        original="y RSHIFT 2 -> g",
    )

    assert parse_instruction("NOT x -> h") == Instruction(
        sources=["x"],
        destination="h",
        gate=Gate.NOT,
        original="NOT x -> h",
    )

    assert parse_instruction("NOT y -> i") == Instruction(
        sources=["y"],
        destination="i",
        gate=Gate.NOT,
        original="NOT y -> i",
    )


def test_solve_basic() -> None:
    instructions_map = {
        "a": parse_instruction("123 -> a"),
        "b": parse_instruction("a -> b"),
        "c": parse_instruction("b -> c"),
    }

    assert solve(instructions_map, "c") == 123


def test_solve_with_examples() -> None:
    instructions_map = {
        "x": parse_instruction("123 -> x"),
        "y": parse_instruction("456 -> y"),
        "d": parse_instruction("x AND y -> d"),
        "e": parse_instruction("x OR y -> e"),
        "f": parse_instruction("x LSHIFT 2 -> f"),
        "g": parse_instruction("y RSHIFT 2 -> g"),
        "h": parse_instruction("NOT x -> h"),
        "i": parse_instruction("NOT y -> i"),
    }

    # Basic
    assert solve(instructions_map, "x") == 123
    assert solve(instructions_map, "y") == 456

    # Only NOT
    assert solve(instructions_map, "h") == 65412
    assert solve(instructions_map, "i") == 65079

    # AND + OR
    assert solve(instructions_map, "d") == 72
    assert solve(instructions_map, "e") == 507

    # SHIFTS
    assert solve(instructions_map, "f") == 492
    assert solve(instructions_map, "g") == 114
