import math
import re
from collections import deque
from typing import Deque, Dict, List, Tuple

from advent_of_code import Problem

PATTERN = re.compile(
    r"(?P<key>[a-z ]+): (?P<lower_start>\d+)-(?P<lower_end>\d+) or (?P<upper_start>\d+)-(?P<upper_end>\d+)"
)

Rules = Dict[str, Tuple[range, range]]
Ticket = List[int]
ValueMap = Dict[str, int]


def parse_rules(lines: List[str]) -> Rules:
    rules: Rules = {}
    for line in lines:
        if not line:
            return rules
        if match := PATTERN.search(line):
            rules[match.group("key")] = (
                range(int(match.group("lower_start")), int(match.group("lower_end")) + 1),
                range(int(match.group("upper_start")), int(match.group("upper_end")) + 1),
            )
    raise Exception("Didn't find an empty line??")


def parse_tickets(lines: List[str], header: str) -> List[Ticket]:
    header_found = False
    tickets: List[List[int]] = []
    for line in lines:
        if header_found:
            if not line:
                break
            tickets.append([int(num) for num in line.split(",")])

        elif line == header:
            header_found = True
    return tickets


def get_invalid_values(rules: Rules, ticket: List[int]) -> List[int]:
    all_ranges = [range_ for ranges in rules.values() for range_ in ranges]

    return list(filter(lambda val: not any(val in range_ for range_ in all_ranges), ticket))


def solve_ticket_map(rules: Rules, tickets: List[Ticket]) -> ValueMap:
    """Solve the ticket values from all known tickets

    1. Create a map of all valid columns for each rule
    2. Go through the map, removing the columns from other rules if there's a
       1:1 relation
    3. Repeat step 2 until only 1 column per rule

    So if we have
      "Rule a": [0, 1],
      "Rule b": [0],
    we remove 0 from rule a, because it can only be for rule b
    """
    solution: ValueMap = {}
    queue: Deque[Tuple[str, List[int]]] = deque()

    columns = list(zip(*tickets))

    # 1. Prefill with all possible solutions
    for rule, ranges in rules.items():
        range_1, range_2 = ranges
        values = [
            idx
            for idx, column in enumerate(columns)
            if all(elm in range_1 or elm in range_2 for elm in column)
        ]
        if len(values) == 1:
            solution[rule] = values[0]
        else:
            queue.append((rule, values))

    # 2. Remove index from multiple lists if there's a 1:1 relation
    while len(queue):
        rule, values = queue.popleft()
        filtered_values = [value for value in values if value not in solution.values()]
        if len(filtered_values) == 1:
            solution[rule] = filtered_values[0]
        else:
            queue.append((rule, filtered_values))

    return solution


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        rules = parse_rules(puzzle_input)
        nearby_tickets = parse_tickets(puzzle_input, "nearby tickets:")

        return sum(sum(get_invalid_values(rules, ticket)) for ticket in nearby_tickets)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        rules = parse_rules(puzzle_input)
        your_ticket = parse_tickets(puzzle_input, "your ticket:")[0]
        nearby_tickets = parse_tickets(puzzle_input, "nearby tickets:")

        filtered_tickets = [
            ticket for ticket in nearby_tickets if not get_invalid_values(rules, ticket)
        ]

        ticket_map = solve_ticket_map(rules, [your_ticket] + filtered_tickets)

        indexes = [value for key, value in ticket_map.items() if key.startswith("departure")]

        return math.prod(your_ticket[idx] for idx in indexes)
