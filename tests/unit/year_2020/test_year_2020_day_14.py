from advent_of_code.year_2020.day_14 import (
    apply_mask_to_value,
    get_addresses_from_mask,
)


def test_apply_mask_to_value() -> None:
    assert apply_mask_to_value("XX1100", 0b010101) == 0b011100
    assert apply_mask_to_value("XXXXXX", 0b010101) == 0b010101
    assert apply_mask_to_value("000000", 0b010101) == 0b000000
    assert apply_mask_to_value("111111", 0b010101) == 0b111111


def test_get_addresses_from_mask() -> None:
    mask = "1X0X1"
    bits = 0b01101

    addresses = list(get_addresses_from_mask(mask, bits))

    assert addresses == [
        0b10101,
        0b11101,
        0b10111,
        0b11111,
    ]
