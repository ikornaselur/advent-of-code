import re

from advent_of_code import Problem

REQUIRED = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
OPTIONAL = ["cid"]

PATTERN = re.compile(r"(?P<key>[a-z]{3}):(?P<value>[#a-z0-9]+)")
HCL_PATTERN = re.compile(r"#[a-f0-9]{6}")
PID_PATTERN = re.compile(r"^[0-9]{9}$")

ECL_VALUES = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}


def validate_entry(entry: str) -> bool:
    for key, value in PATTERN.findall(entry):
        if key == "byr":
            val = int(value)
            if val < 1920 or val > 2002:
                return False
        elif key == "iyr":
            val = int(value)
            if val < 2010 or val > 2020:
                return False
        elif key == "eyr":
            val = int(value)
            if val < 2020 or val > 2030:
                return False
        elif key == "hgt":
            if value[-2:] not in ("cm", "in"):
                return False
            val = int(value[:-2])
            if value[-2:] == "cm" and (val < 150 or val > 193):
                return False
            elif value[-2:] == "in" and (val < 59 or val > 76):
                return False
        elif key == "hcl":
            match = HCL_PATTERN.search(value)
            if match is None:
                return False
        elif key == "ecl":
            if value not in ECL_VALUES:
                return False
        elif key == "pid":
            match = PID_PATTERN.search(value)
            if match is None:
                return False
        elif key == "cid":
            pass
        else:
            raise Exception(f"Unknown key: {key}")
    return True


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)
        entries = [entry.replace("\n", " ") for entry in "".join(puzzle_input).split("\n\n")]

        valid = 0

        for entry in entries:
            keys = [key for item in entry.split() if (key := item.split(":")[0]) in REQUIRED]
            if len(keys) == len(REQUIRED):
                valid += 1

        return valid


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)
        entries = [entry.replace("\n", " ") for entry in "".join(puzzle_input).split("\n\n")]

        valid = 0

        for entry in entries:
            keys = [key for item in entry.split() if (key := item.split(":")[0]) in REQUIRED]
            if len(keys) != len(REQUIRED):
                continue
            if validate_entry(entry):
                valid += 1

        return valid
