from advent_of_code.year_2015.day_6 import Action, Part1


def test_parse_instruction() -> None:
    part = Part1(width=10, height=10)

    assert part.parse_instruction("turn on 10,10 through 20,20") == (Action.on, (10, 10), (20, 20))
    assert part.parse_instruction("turn off 15,10 through 15,20") == (
        Action.off,
        (15, 10),
        (15, 20),
    )
    assert part.parse_instruction("toggle 0,0 through 999,999") == (
        Action.toggle,
        (0, 0),
        (999, 999),
    )


def test_execute_instruction() -> None:
    part = Part1(width=3, height=3)

    assert part._lights == [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ]

    part.execute_instruction((Action.on, (0, 0), (2, 1)))

    assert part._lights == [
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
    ]

    part.execute_instruction((Action.off, (1, 0), (2, 1)))

    assert part._lights == [
        [1, 1, 0],
        [0, 0, 0],
        [0, 0, 0],
    ]

    part.execute_instruction((Action.toggle, (0, 1), (1, 2)))

    assert part._lights == [
        [1, 0, 1],
        [0, 1, 1],
        [0, 0, 0],
    ]
