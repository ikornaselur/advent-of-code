from advent_of_code.year_2020.day_17 import (
    get_active_cubes,
    get_corners,
    initialise_world,
    print_world,
    tick,
)


def test_initialise_world() -> None:
    zero_slice = [
        ".#.",
        "..#",
        "###",
    ]
    world = initialise_world(zero_slice)
    assert world == {
        (0, 0, 0): False,
        (0, 1, 0): True,
        (0, 2, 0): False,
        (1, 0, 0): False,
        (1, 1, 0): False,
        (1, 2, 0): True,
        (2, 0, 0): True,
        (2, 1, 0): True,
        (2, 2, 0): True,
    }


def test_print_world() -> None:
    world = {
        (0, 0, 0): False,
        (0, 1, 0): True,
        (0, 2, 0): False,
        (1, 0, 0): False,
        (1, 1, 0): False,
        (1, 2, 0): True,
        (2, 0, 0): True,
        (2, 1, 0): True,
        (2, 2, 0): True,
    }

    output = print_world(world)

    assert output == "\n".join(
        [
            "z=0",
            ".#.",
            "..#",
            "###",
            "",
        ]
    )


def test_get_corners() -> None:
    world = {
        (0, 0, 10): True,
        (1, 0, -3): True,
        (-5, 2, 0): True,
    }

    min_corner, max_corner = get_corners(world)
    assert min_corner == (-5, 0, -3)
    assert max_corner == (1, 2, 10)


def test_get_active_cubes() -> None:
    world = {
        (0, 0, 0): False,
        (0, 1, 0): True,
        (0, 2, 0): False,
        (1, 0, 0): False,
        (1, 1, 0): False,
        (1, 2, 0): True,
        (2, 0, 0): True,
        (2, 1, 0): True,
        (2, 2, 0): True,
    }

    assert get_active_cubes((0, 0, 0), world) == 1
    assert get_active_cubes((0, 1, 0), world) == 1
    assert get_active_cubes((0, 2, 0), world) == 2


def test_tick() -> None:
    world = {
        (0, 0, 0): False,
        (0, 1, 0): True,
        (0, 2, 0): False,
        (1, 0, 0): False,
        (1, 1, 0): False,
        (1, 2, 0): True,
        (2, 0, 0): True,
        (2, 1, 0): True,
        (2, 2, 0): True,
    }

    new_world = tick(world)

    print(print_world(new_world))
    assert print_world(new_world) == "\n".join(
        [
            "z=-1",
            "#..",
            "..#",
            ".#.",
            "",
            "z=0",
            "#.#",
            ".##",
            ".#.",
            "",
            "z=1",
            "#..",
            "..#",
            ".#.",
            "",
        ]
    )


def test_boot_cycle() -> None:
    initial_slice = [
        ".#.",
        "..#",
        "###",
    ]

    world = initialise_world(initial_slice)

    for _ in range(6):
        world = tick(world)

    assert sum(world.values()) == 112
