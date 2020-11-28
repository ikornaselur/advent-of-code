from advent_of_code.year_2015.day_8 import total_chars, total_memory, encode


def test_total_chars() -> None:
    assert total_chars(r'""') == 2
    assert total_chars(r'"abc"') == 5
    assert total_chars(r'"aaa\"aaa"') == 10
    assert total_chars(r'"\x27"') == 6


def test_total_memory() -> None:
    assert total_memory(r'""') == 0
    assert total_memory(r'"abc"') == 3
    assert total_memory(r'"aaa\"aaa"') == 7
    assert total_memory(r'"\x27"') == 1


def test_encode() -> None:
    assert total_chars(encode(r'""')) == 6
    assert total_chars(encode(r'"abc"')) == 9
    assert total_chars(encode(r'"aaa\"aaa"')) == 16
    assert total_chars(encode(r'"\x27"')) == 11
