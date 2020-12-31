from advent_of_code.year_2020.day_20 import Tile, find_corners


def test_tile_init() -> None:
    raw_tile = [
        "Tile 999:",
        "123",
        "456",
        "789",
    ]

    tile = Tile(raw_tile)

    assert tile.id_num == 999

    assert tile.sides == {
        "123",
        "321",
        "789",
        "987",
        "147",
        "741",
        "369",
        "963",
    }


def test_tile_adjacent() -> None:
    tile_a = Tile(
        [
            "Tile 123:",
            "123",
            "456",
            "789",
        ]
    )

    tile_b = Tile(
        [
            "Tile 123:",
            "...",
            "...",
            "1..",
        ]
    )

    tile_c = Tile(
        [
            "Tile 123:",
            "..3",
            "..2",
            "..1",
        ]
    )

    assert not tile_a.adjacent(tile_b)
    assert tile_a.adjacent(tile_c)
    assert tile_b.adjacent(tile_c)


def test_find_corners() -> None:
    """
    Test tiles are going to be 2x2 wide for a total of 3x3 tiles. Overall map is:

        12 23 34
        56 67 78

        56 67 78
        90 0a ab

        90 0a ab
        cd de ef
    """

    tiles = [
        Tile(["Tile 1:", "12", "56"]),
        Tile(["Tile 2:", "23", "67"]),
        Tile(["Tile 3:", "34", "78"]),
        Tile(["Tile 4:", "56", "90"]),
        Tile(["Tile 5:", "67", "0a"]),
        Tile(["Tile 6:", "78", "ab"]),
        Tile(["Tile 7:", "90", "cd"]),
        Tile(["Tile 8:", "0a", "de"]),
        Tile(["Tile 9:", "ab", "ef"]),
    ]

    corners = find_corners(tiles)

    assert len(corners) == 4

    assert {corner.id_num for corner in corners} == {1, 3, 7, 9}
