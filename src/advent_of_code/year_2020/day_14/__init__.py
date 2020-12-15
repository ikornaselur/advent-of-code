import re
from typing import Dict, Iterator

from advent_of_code import Problem

PATTERN = re.compile(r"mem\[(?P<addr>\d+)\] = (?P<value>\d+)")


def apply_mask_to_value(mask: str, bits: int) -> int:
    one_mask = int(mask.replace("X", "0"), 2)
    zero_mask = int(mask.replace("X", "1"), 2)

    return bits & zero_mask | one_mask


def get_addresses_from_mask(mask: str, bits: int) -> Iterator[int]:
    one_mask = int(mask.replace("X", "0"), 2)
    masked_bits = bits | one_mask

    def generate_options(sub_mask: str, sub_bits: str) -> Iterator[str]:
        if len(sub_mask) == 1:
            if sub_mask == "X":
                yield "0"
                yield "1"
            else:
                yield sub_bits
        else:
            for sub in generate_options(sub_mask[1:], sub_bits[1:]):
                if sub_mask[0] == "X":
                    yield "0" + sub
                    yield "1" + sub
                else:
                    yield sub_bits[0] + sub

    for option in generate_options(mask, f"{masked_bits:0{len(mask)}b}"):
        yield int(option, 2)


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        mask = ""
        mem: Dict[int, int] = {}

        for line in puzzle_input:
            if line.startswith("mask"):
                mask = line.split(" = ")[-1]
                continue

            if match := PATTERN.search(line):
                addr = int(match.group("addr"))
                value = int(match.group("value"))

                mem[addr] = apply_mask_to_value(mask, value)

        return sum(mem.values())


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        mask = ""
        mem: Dict[int, int] = {}

        for line in puzzle_input:
            if line.startswith("mask"):
                mask = line.split(" = ")[-1]
                continue

            if match := PATTERN.search(line):
                addr = int(match.group("addr"))
                value = int(match.group("value"))

                for sub_addr in get_addresses_from_mask(mask, addr):
                    mem[sub_addr] = value

        return sum(mem.values())
