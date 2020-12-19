from advent_of_code.year_2020.day_17 import Simulation3D


def test_initialise_world() -> None:
    zero_slice = [
        ".#.",
        "..#",
        "###",
    ]
    simulation = Simulation3D(zero_slice)
    assert simulation.world == {
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
    simulation = Simulation3D([])
    simulation.world = {
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

    output = simulation.print_sim()

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
    simulation = Simulation3D([])
    simulation.world = {
        (0, 0, 10): True,
        (1, 0, -3): True,
        (-5, 2, 0): True,
    }

    min_corner, max_corner = simulation.get_corners()
    assert min_corner == (-5, 0, -3)
    assert max_corner == (1, 2, 10)


def test_get_active_cubes() -> None:
    simulation = Simulation3D([])
    simulation.world = {
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

    assert simulation.get_active_cubes((0, 0, 0)) == 1
    assert simulation.get_active_cubes((0, 1, 0)) == 1
    assert simulation.get_active_cubes((0, 2, 0)) == 2


def test_tick() -> None:
    simulation = Simulation3D([])
    simulation.world = {
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

    simulation.tick()

    assert simulation.print_sim() == "\n".join(
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

    simulation = Simulation3D(initial_slice)

    for _ in range(6):
        simulation.tick()

    assert sum(simulation.world.values()) == 112
