from advent_of_code.year_2020.day_18 import (
    calculate_no_precedence,
    calculate_reverse_precedence,
    tokenise,
)


def test_calculate_no_precedence() -> None:
    assert calculate_no_precedence(tokenise("2 * 3 + (4 * 5)")) == 26
    assert calculate_no_precedence(tokenise("5 + (8 * 3 + 9 + 3 * 4 * 3)")) == 437
    assert calculate_no_precedence(tokenise("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")) == 12240
    assert (
        calculate_no_precedence(tokenise("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"))
        == 13632
    )


def test_calculate_reverse_precedence() -> None:
    assert calculate_reverse_precedence(tokenise("1 + (2 * 3) + (4 * (5 + 6))")) == 51
    assert calculate_reverse_precedence(tokenise("2 * 3 + (4 * 5)")) == 46
    assert calculate_reverse_precedence(tokenise("5 + (8 * 3 + 9 + 3 * 4 * 3)")) == 1445
    assert (
        calculate_reverse_precedence(tokenise("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"))
        == 669060
    )
    assert (
        calculate_reverse_precedence(tokenise("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"))
        == 23340
    )
