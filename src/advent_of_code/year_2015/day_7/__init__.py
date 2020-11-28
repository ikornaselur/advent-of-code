import re
from dataclasses import dataclass
from enum import Enum, auto
from functools import lru_cache
from typing import Dict, List, Optional, Union

from advent_of_code import Problem


class Gate(Enum):
    AND = auto()
    OR = auto()
    LSHIFT = auto()
    RSHIFT = auto()
    NOT = auto()
    NONE = auto()


@dataclass
class Instruction:
    sources: List[Union[str, int]]
    destination: str
    gate: Gate
    original: str

    def __repr__(self: "Instruction") -> str:
        return self.original


InstructionsMap = Dict[str, Instruction]


PATTERN = re.compile(
    r"((?P<source2>[a-z0-9]+) )?((?P<gate>[A-Z]+) )?(?P<source1>[a-zA-Z0-9]+) -> (?P<destination>[a-zA-Z]+)"
)


def parse_instruction(instruction: str) -> Instruction:
    match = PATTERN.search(instruction)
    if not match:
        raise Exception("Unable to parse instruction")

    if raw_gate := match.group("gate"):
        gate = getattr(Gate, raw_gate.upper())
    else:
        gate = Gate.NONE

    source2: Optional[Union[str, int]] = None
    if (source2_raw := match.group("source2")) is not None:
        if source2_raw.isdigit():
            source2 = int(source2_raw)
        else:
            source2 = source2_raw

    source1: Union[str, int]
    source1_raw = match.group("source1")
    if source1_raw.isdigit():
        source1 = int(source1_raw)
    else:
        source1 = source1_raw

    return Instruction(
        sources=[s for s in [source2, source1] if s is not None],
        gate=gate,
        destination=match.group("destination"),
        original=instruction,
    )


def solve(instructions_map: InstructionsMap, destination: str) -> int:
    @lru_cache
    def _solve(dest: str) -> int:
        instruction = instructions_map[dest]

        source1_raw: Union[str, int]
        source1: int
        source2_raw: Optional[Union[str, int]]
        source2: Optional[int]

        if len(instruction.sources) == 1:
            source1_raw = instruction.sources[0]
            source2_raw = None
        else:
            source2_raw, source1_raw = instruction.sources

        if isinstance(source1_raw, str):
            source1 = _solve(source1_raw)
        else:
            source1 = source1_raw
        if isinstance(source2_raw, str):
            source2 = _solve(source2_raw)
        else:
            source2 = source2_raw

        if instruction.gate == Gate.NONE:
            return source1
        elif instruction.gate == Gate.NOT:
            # Invert lowermost 16 bits
            return ~source1 & 0xFFFF
        elif source2 is not None:
            if instruction.gate == Gate.AND:
                return source2 & source1
            elif instruction.gate == Gate.OR:
                return source2 | source1
            elif instruction.gate == Gate.LSHIFT:
                return source2 << source1
            elif instruction.gate == Gate.RSHIFT:
                return source2 >> source1
        raise Exception("Unable to solve")

    return _solve(destination)


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        instructions_map: InstructionsMap = {}

        # Populate initial map
        for line in puzzle_input:
            instruction = parse_instruction(line)
            instructions_map[instruction.destination] = parse_instruction(line)

        return solve(instructions_map, "a")


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        instructions_map: InstructionsMap = {}

        # Populate initial map
        for line in puzzle_input:
            instruction = parse_instruction(line)
            instructions_map[instruction.destination] = parse_instruction(line)

        initial_value = solve(instructions_map, "a")
        instructions_map["b"].sources = [initial_value]

        return solve(instructions_map, "a")
