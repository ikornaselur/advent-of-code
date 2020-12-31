import re
from functools import lru_cache
from typing import Dict, List

from advent_of_code import Problem

VALID_RULE = re.compile(r'^"?[ab\(\)\| ]+"?$')


def parse_rule(rules: List[str], rule_num: int) -> str:
    raw: Dict[int, str] = {int(rule[0]): rule[1] for row in rules if (rule := row.split(": "))}

    @lru_cache
    def _parse_rule(rule_num: int) -> str:
        rule = raw[rule_num]
        wrap = "|" in rule

        if VALID_RULE.match(rule):
            if '"' in rule:
                return rule[1:-1]
            return f"({rule})" if wrap else rule

        refs = re.findall(r"\d+", rule)
        ref_map = {ref: _parse_rule(int(ref)) for ref in refs}
        for ref, val in ref_map.items():
            rule = re.sub(r"\b" + ref + r"\b", val, rule)
        return f"({rule})" if wrap else rule

    return _parse_rule(rule_num).replace(" ", "")


def count_valid_strings(strings: List[str], rule: str) -> int:
    return len([string for string in strings if re.match(f"^{rule}$", string)])


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        split = puzzle_input.index("")

        rule = parse_rule(puzzle_input[:split], 0)

        return count_valid_strings(puzzle_input[split + 1 :], rule)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
