from advent_of_code.year_2020.day_04 import validate_entry


def test_validate_entry_byr() -> None:
    assert validate_entry("byr:99999") is False
    assert validate_entry("byr:9999") is False
    assert validate_entry("byr:999") is False
    assert validate_entry("byr:1919") is False
    assert validate_entry("byr:2003") is False

    assert validate_entry("byr:1920") is True
    assert validate_entry("byr:1921") is True
    assert validate_entry("byr:1999") is True
    assert validate_entry("byr:2001") is True
    assert validate_entry("byr:2002") is True


def test_validate_entry_iyr() -> None:
    assert validate_entry("iyr:99999") is False
    assert validate_entry("iyr:9999") is False
    assert validate_entry("iyr:999") is False
    assert validate_entry("iyr:2009") is False
    assert validate_entry("iyr:2021") is False

    assert validate_entry("iyr:2010") is True
    assert validate_entry("iyr:2011") is True
    assert validate_entry("iyr:2019") is True
    assert validate_entry("iyr:2020") is True


def test_validate_entry_eyr() -> None:
    assert validate_entry("eyr:99999") is False
    assert validate_entry("eyr:9999") is False
    assert validate_entry("eyr:999") is False
    assert validate_entry("eyr:2019") is False
    assert validate_entry("eyr:2031") is False

    assert validate_entry("eyr:2020") is True
    assert validate_entry("eyr:2021") is True
    assert validate_entry("eyr:2029") is True
    assert validate_entry("eyr:2030") is True


def test_validate_entry_hgt() -> None:
    assert validate_entry("hgt:100") is False
    assert validate_entry("hgt:149cm") is False
    assert validate_entry("hgt:194cm") is False
    assert validate_entry("hgt:58in") is False
    assert validate_entry("hgt:77in") is False

    assert validate_entry("hgt:150cm") is True
    assert validate_entry("hgt:193cm") is True
    assert validate_entry("hgt:59in") is True
    assert validate_entry("hgt:76in") is True


def test_validate_entry_hcl() -> None:
    assert validate_entry("hcl:000000") is False
    assert validate_entry("hcl:abcdef") is False
    assert validate_entry("hcl:#123") is False
    assert validate_entry("hcl:#12345") is False
    assert validate_entry("hcl:#12345g") is False

    assert validate_entry("hcl:#000000") is True
    assert validate_entry("hcl:#ffffff") is True
    assert validate_entry("hcl:#012345") is True
    assert validate_entry("hcl:#abcdef") is True


def test_validate_entry_ecl() -> None:
    assert validate_entry("ecl:wat") is False
    assert validate_entry("ecl:am") is False
    assert validate_entry("ecl:blue") is False

    assert validate_entry("ecl:amb") is True
    assert validate_entry("ecl:blu") is True
    assert validate_entry("ecl:brn") is True
    assert validate_entry("ecl:gry") is True
    assert validate_entry("ecl:grn") is True
    assert validate_entry("ecl:hzl") is True
    assert validate_entry("ecl:oth") is True


def test_validate_entry_pid() -> None:
    assert validate_entry("pid:00000001") is False
    assert validate_entry("pid:0000000001") is False

    assert validate_entry("pid:000000001") is True
    assert validate_entry("pid:012345678") is True
