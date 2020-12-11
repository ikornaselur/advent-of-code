from advent_of_code.year_2015.day_5 import Part1, Part2


def test_three_vowels() -> None:
    part = Part1()

    assert part._has_three_vowels("") is False
    assert part._has_three_vowels("bcdfghjkli") is False
    assert part._has_three_vowels("bbbbbbbbb") is False
    assert part._has_three_vowels("abba") is False
    assert part._has_three_vowels("ae") is False

    assert part._has_three_vowels("aei") is True
    assert part._has_three_vowels("ammmmemmmmmi") is True


def test_repeated_letters() -> None:
    part = Part1()

    assert part._has_repeated_letters("") is False
    assert part._has_repeated_letters("abc") is False
    assert part._has_repeated_letters("abcabdabc") is False

    assert part._has_repeated_letters("abccba") is True
    assert part._has_repeated_letters("bb") is True


def test_forbidden_tuples() -> None:
    part = Part1()

    assert part._has_forbidden_tuples("") is False
    assert part._has_forbidden_tuples("mmmmmmm") is False
    assert part._has_forbidden_tuples("xzy") is False

    assert part._has_forbidden_tuples("abc") is True
    assert part._has_forbidden_tuples("just use cd") is True
    assert part._has_forbidden_tuples("xyz") is True


def test_non_overlapping_pair() -> None:
    part = Part2()

    assert part._has_non_overlapping_pair("") is False
    assert part._has_non_overlapping_pair("aaa") is False
    assert part._has_non_overlapping_pair("aabb") is False
    assert part._has_non_overlapping_pair("ieodomkazucvgmuy") is False

    assert part._has_non_overlapping_pair("aaaa") is True
    assert part._has_non_overlapping_pair("xyxy") is True
    assert part._has_non_overlapping_pair("xxyxx") is True
    assert part._has_non_overlapping_pair("aabcdefgaa") is True
    assert part._has_non_overlapping_pair("qjhvhtzxzqqjkmpb") is True
    assert part._has_non_overlapping_pair("uurcxstgmygtbstg") is True


def test_repeated_one_letter_apart() -> None:
    part = Part2()

    assert part._has_repeat_one_letter_apart("") is False
    assert part._has_repeat_one_letter_apart("xx") is False
    assert part._has_repeat_one_letter_apart("uurcxstgmygtbstg") is False

    assert part._has_repeat_one_letter_apart("xxx") is True
    assert part._has_repeat_one_letter_apart("xyx") is True
    assert part._has_repeat_one_letter_apart("xxyxx") is True
    assert part._has_repeat_one_letter_apart("abcdefeghi") is True
    assert part._has_repeat_one_letter_apart("qjhvhtzxzqqjkmpb") is True
    assert part._has_repeat_one_letter_apart("ieodomkazucvgmuy") is True
