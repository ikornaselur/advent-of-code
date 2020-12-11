from advent_of_code.year_2020.day_03 import traverse_map


def test_traverse_map() -> None:
    tree_map = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ]

    assert traverse_map(tree_map, movement=(1, 1)) == 2
    assert traverse_map(tree_map, movement=(3, 1)) == 7
    assert traverse_map(tree_map, movement=(5, 1)) == 3
    assert traverse_map(tree_map, movement=(7, 1)) == 4
    assert traverse_map(tree_map, movement=(1, 2)) == 2
