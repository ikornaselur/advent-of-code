from advent_of_code.year_2020.day_12 import ShipPosition


def test_move() -> None:
    pos = ShipPosition(1, 2, 3)

    pos.move("N10")
    assert pos == ShipPosition(11, 2, 3)

    pos.move("S3")
    assert pos == ShipPosition(8, 2, 3)

    pos.move("W5")
    assert pos == ShipPosition(8, -3, 3)

    pos.move("E9")
    assert pos == ShipPosition(8, 6, 3)

    pos.move("L90")
    assert pos == ShipPosition(8, 6, 2)

    pos.move("F1")
    assert pos == ShipPosition(7, 6, 2)

    pos.move("R180")
    assert pos == ShipPosition(7, 6, 0)

    pos.move("F1")
    assert pos == ShipPosition(8, 6, 0)
