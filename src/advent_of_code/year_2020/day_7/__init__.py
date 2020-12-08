import re
from typing import Dict, List, Set, Tuple

from advent_of_code import Problem

PATTERN = re.compile(r"\d+ ([\w\s]+) bags?")


def parse_rule(rule: str) -> Tuple[str, List[str]]:
    container, bags_str = rule.split(" bags contain ")

    if bags_str == "no other bags":
        bags = []
    else:
        bags = PATTERN.findall(bags_str)

    return container, bags


def create_map(rules: List[Tuple[str, List[str]]]) -> Dict[str, Set[str]]:
    """ Create a map of potential parents """
    bag_map: Dict[str, Set[str]] = {}

    for parent, children in rules:
        for child in children:
            if child not in bag_map:
                bag_map[child] = set()
            bag_map[child].add(parent)

    return bag_map


def can_contain(bag: str, rules: List[Tuple[str, List[str]]]) -> int:
    bag_map = create_map(rules)

    top_level_bags: Set[str] = set()
    checked_bags: Set[str] = set()

    queue: List[str] = [bag]

    while len(queue):
        next_bag = queue.pop()
        if next_bag not in bag_map:
            continue
        parent_bags = bag_map[next_bag]
        for parent_bag in parent_bags:
            if parent_bag in checked_bags:
                continue
            checked_bags.add(parent_bag)
            queue.append(parent_bag)
            top_level_bags.add(parent_bag)

    return len(top_level_bags)


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [line.strip() for line in self.get_input(__file__)]

        rules = list(map(parse_rule, puzzle_input))

        return can_contain("shiny gold", rules)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return len(puzzle_input) * 0
