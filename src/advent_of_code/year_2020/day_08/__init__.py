import re
from enum import Enum
from typing import List, Set, Tuple

from advent_of_code import Problem


class Instruction(Enum):
    jmp = "jmp"
    nop = "nop"
    acc = "acc"


class OutOfBounds(Exception):
    pass


PATTERN = re.compile(r"(?P<instruction>jmp|nop|acc) (?P<value>[+-]\d+)")


def parse_instruction(instruction: str) -> Tuple[Instruction, int]:
    match = PATTERN.search(instruction)
    if match is None:
        raise Exception(f"Invalid instruction: {instruction}")

    return Instruction(match.group("instruction")), int(match.group("value"))


def run_instructions(instructions: List[Tuple[Instruction, int]]) -> Tuple[int, bool]:
    accumulator = 0
    visited_lines: Set[int] = set()
    instruction_count = len(instructions)
    pointer = 0

    while True:
        if pointer in visited_lines:
            return accumulator, False  # Didn't finish running
        if pointer == instruction_count:
            return accumulator, True  # Ran successfully
        if pointer > instruction_count:
            raise OutOfBounds("Pointer went beyond the program")

        visited_lines.add(pointer)

        instruction, value = instructions[pointer]
        if instruction == Instruction.nop:
            pointer += 1
        elif instruction == Instruction.acc:
            accumulator += value
            pointer += 1
        elif instruction == Instruction.jmp:
            pointer += value


def get_visited_pointers(instructions: List[Tuple[Instruction, int]]) -> Set[int]:
    accumulator = 0
    visited_lines: Set[int] = set()
    pointer = 0

    while True:
        if pointer in visited_lines:
            return visited_lines
        visited_lines.add(pointer)

        instruction, value = instructions[pointer]
        if instruction == Instruction.nop:
            pointer += 1
        elif instruction == Instruction.acc:
            accumulator += value
            pointer += 1
        elif instruction == Instruction.jmp:
            pointer += value


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        instructions = list(map(parse_instruction, puzzle_input))

        return run_instructions(instructions)[0]


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        instructions = list(map(parse_instruction, puzzle_input))

        visited_pointers = get_visited_pointers(instructions)

        to_test_pointers = {
            pointer
            for pointer in visited_pointers
            if instructions[pointer][0] in (Instruction.nop, Instruction.jmp)
        }

        for pointer in to_test_pointers:
            # Test them one by one from the original loop
            original_instruction, value = instructions[pointer]

            if original_instruction == Instruction.nop:
                instructions[pointer] = (Instruction.jmp, value)
            else:
                instructions[pointer] = (Instruction.nop, value)

            accumulator, successful = run_instructions(instructions)
            if successful:
                return accumulator

            instructions[pointer] = (original_instruction, value)

        return len(puzzle_input) * 0
