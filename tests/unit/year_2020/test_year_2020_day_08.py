from advent_of_code.year_2020.day_08 import (
    Instruction,
    parse_instruction,
    run_instructions,
)


def test_parse_instruction() -> None:
    assert parse_instruction("acc +3") == (Instruction("acc"), 3)
    assert parse_instruction("jmp -44") == (Instruction("jmp"), -44)
    assert parse_instruction("nop +255") == (Instruction("nop"), 255)


def test_run_instructions_loop() -> None:
    instructions = list(
        map(
            parse_instruction,
            [
                "nop +0",
                "acc +1",
                "jmp +4",
                "acc +3",
                "jmp -3",
                "acc -99",
                "acc +1",
                "jmp -4",
                "acc +6",
            ],
        )
    )

    assert run_instructions(instructions) == (5, False)


def test_run_instructions_successful() -> None:
    instructions = list(
        map(
            parse_instruction,
            [
                "nop +0",
                "acc +1",
                "jmp +4",
                "acc +3",
                "jmp -3",
                "acc -99",
                "acc +1",
                "nop -4",
                "acc +6",
            ],
        )
    )

    assert run_instructions(instructions) == (8, True)
