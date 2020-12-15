from advent_of_code.year_2020.day_13 import (
    find_earliest_bus,
    find_earliest_subsequent,
    two_bus_departure,
)


def test_find_earliest_bus() -> None:
    earliest_departure = 939
    buses = [7, 13, 59, 31, 19]

    bus, departure = find_earliest_bus(earliest_departure, buses)

    assert bus == 59
    assert departure == 944


def test_find_earliest_subsequent() -> None:
    assert find_earliest_subsequent("1,2") == 1
    assert find_earliest_subsequent("5,4") == 15
    assert find_earliest_subsequent("5,x,4") == 10
    assert find_earliest_subsequent("1,2,3") == 1
    assert find_earliest_subsequent("1,x,2") == 0
    assert find_earliest_subsequent("1,3") == 2
    assert find_earliest_subsequent("1,2,x,3") == 3
    assert find_earliest_subsequent("2,x,4,9") == 6
    assert find_earliest_subsequent("7,x,5,11") == 63
    assert find_earliest_subsequent("3,2,1") == 3
    assert find_earliest_subsequent("5,x,4,x,3") == 50
    assert find_earliest_subsequent("11,x,7,x,3") == 110
    assert find_earliest_subsequent("3,x,7,x,11") == 117

    assert find_earliest_subsequent("3,x,x,x,x,5") == 0
    assert find_earliest_subsequent("3,x,x,x,x,x,5") == 9

    assert find_earliest_subsequent("19,41") == 532
    assert find_earliest_subsequent("19,x,41") == 285
    assert find_earliest_subsequent("19,x,x,41") == 38
    assert find_earliest_subsequent("19,x,x,x,41") == 570
    assert find_earliest_subsequent("19,x,x,x,x,41") == 323
    assert find_earliest_subsequent("19,x,x,x,x,x,41") == 76
    assert find_earliest_subsequent("19,x,x,x,x,x,x,41") == 608
    assert find_earliest_subsequent("19,x,x,x,x,x,x,x,41") == 361
    assert find_earliest_subsequent("19,x,x,x,x,x,x,x,x,41") == 114

    assert find_earliest_subsequent("x,5,x,3") == 9
    assert find_earliest_subsequent("x,5,x,3,x,2") == 9
    assert find_earliest_subsequent("x,5,x,3,x,7") == 9
    assert find_earliest_subsequent("x,5,x,4,x,3") == 49

    assert find_earliest_subsequent("1000,2000") == 1000


def test_two_bus_departure() -> None:
    assert two_bus_departure((2, 0), (5, 2)) == (10, 8)
    assert two_bus_departure((2, 0), (3, 3)) == (6, 0)
    assert two_bus_departure((7, 0), (5, 2)) == (35, 28)
    assert two_bus_departure((7, 0), (11, 3)) == (77, 63)

    assert two_bus_departure((4, 2), (9, 3)) == (36, 6)
