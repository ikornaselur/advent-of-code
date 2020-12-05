from advent_of_code.year_2020.day_5 import decode_seat


def test_decode_seat() -> None:
    assert decode_seat("FBFBBFFRLR") == (44, 5)
    assert decode_seat("BFFFBBFRRR") == (70, 7)
    assert decode_seat("FFFBBBFRRR") == (14, 7)
    assert decode_seat("BBFFBBFRLL") == (102, 4)
