import re
from functools import lru_cache
from typing import Dict, List

from advent_of_code import Problem

VALID_RULE = re.compile(r'^"?[ab\(\)\| ]+"?$')


def parse_rule(rules: List[str], rule_num: int, max_cycle: int = 5) -> str:
    raw: Dict[int, str] = {int(rule[0]): rule[1] for row in rules if (rule := row.split(": "))}

    @lru_cache
    def _parse_rule(rule_num: int, max_cycle: int) -> str:
        rule = raw[rule_num]

        if max_cycle <= 0 and re.search(r"\b" + str(rule_num) + r"\b", rule):
            # We've reach the max depth of a cycle, remove the cycle now
            rule_parts = rule.split("|")
            rule = "|".join(
                part for part in rule_parts if not re.search(r"\b" + str(rule_num) + r"\b", part)
            )

        wrap = "|" in rule

        if VALID_RULE.match(rule):
            if '"' in rule:
                return rule[1:-1]
            return f"({rule})" if wrap else rule

        refs = re.findall(r"\d+", rule)
        ref_map = {ref: _parse_rule(int(ref), max_cycle - 1) for ref in refs}
        for ref, val in ref_map.items():
            rule = re.sub(r"\b" + ref + r"\b", val, rule)
        return f"({rule})" if wrap else rule

    return _parse_rule(rule_num, max_cycle).replace(" ", "")


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

        split = puzzle_input.index("")

        rules = puzzle_input[:split]

        # Replace the rules in part 2
        rules[rules.index("8: 42")] = "8: 42 | 42 8"
        rules[rules.index("11: 42 31")] = "11: 42 31 | 42 11 31"

        rule = parse_rule(rules, 0, max_cycle=5)

        return count_valid_strings(puzzle_input[split + 1 :], rule)
