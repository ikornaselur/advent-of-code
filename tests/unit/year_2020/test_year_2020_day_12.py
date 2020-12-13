from advent_of_code.year_2020.day_12 import ShipPosition, WaypointPosition


def test_move_ship_position() -> None:
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


def test_move_waypoint_position() -> None:
    pos = WaypointPosition(0, 0, 1, 10)

    pos.move("F10")
    assert pos == WaypointPosition(10, 100, 1, 10)

    pos.move("N3")
    assert pos == WaypointPosition(10, 100, 4, 10)

    pos.move("F7")
    assert pos == WaypointPosition(38, 170, 4, 10)

    pos.move("R90")
    assert pos == WaypointPosition(38, 170, -10, 4)

    pos.move("F11")
    assert pos == WaypointPosition(-72, 214, -10, 4)
