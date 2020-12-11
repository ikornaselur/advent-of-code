from typing import List, Set

from advent_of_code import Problem


def count_yesses(rows: List[str]) -> int:
    yesses = 0
    answers: Set[str] = set()
    for row in rows:
        if not len(row):
            yesses += len(answers)
            answers = set()
        else:
            answers.update(set(row))
    else:
        yesses += len(answers)

    return yesses


def count_all_answered_yes(rows: List[str]) -> int:
    yesses = 0
    answers: List[Set[str]] = []
    for row in rows:
        if not len(row):
            base = answers[0]
            for ans in answers[1:]:
                base.intersection_update(ans)
            yesses += len(base)
            answers = []
        else:
            answers.append(set(row))
    else:
        base = answers[0]
        for ans in answers[1:]:
            base.intersection_update(ans)
        yesses += len(base)

    return yesses


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [row.strip() for row in self.get_input(__file__)]

        return count_yesses(puzzle_input)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = [row.strip() for row in self.get_input(__file__)]

        return count_all_answered_yes(puzzle_input)
