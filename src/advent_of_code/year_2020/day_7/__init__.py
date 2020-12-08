import re
from functools import lru_cache
from typing import Dict, List, Set, Tuple

from advent_of_code import Problem

CONTAIN_PATTERN = re.compile(r"\d+ ([\w\s]+) bags?")
LEVEL_PATTERN = re.compile(r"(\d+) ([\w\s]+) bags?")

##########
# Part 1 #
##########


def parse_rule(rule: str) -> Tuple[str, List[str]]:
    container, bags_str = rule.split(" bags contain ")

    if bags_str == "no other bags":
        bags = []
    else:
        bags = CONTAIN_PATTERN.findall(bags_str)

    return container, bags


def create_contain_map(rules: List[Tuple[str, List[str]]]) -> Dict[str, Set[str]]:
    """ Create a map of potential parents """
    bag_map: Dict[str, Set[str]] = {}

    for parent, children in rules:
        for child in children:
            if child not in bag_map:
                bag_map[child] = set()
            bag_map[child].add(parent)

    return bag_map


def can_contain(bag: str, rules: List[Tuple[str, List[str]]]) -> int:
    bag_map = create_contain_map(rules)

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


##########
# Part 2 #
##########


def parse_rule_level(rule: str) -> Tuple[str, List[Tuple[int, str]]]:
    container, bags_str = rule.split(" bags contain ")

    if bags_str == "no other bags":
        bags = []
    else:
        bags = [(int(count), bag) for count, bag in LEVEL_PATTERN.findall(bags_str)]

    return container, bags


def bag_levels(bag: str, rules: List[Tuple[str, List[Tuple[int, str]]]]) -> int:
    bag_map = dict(rules)

    @lru_cache
    def _bag_levels(bag: str) -> int:
        sub_bags = bag_map[bag]

        count = 0
        for bag_count, sub_bag in sub_bags:
            sub_bag_count = _bag_levels(sub_bag)
            count += bag_count + bag_count * sub_bag_count

        return count

    return _bag_levels(bag)


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [line.strip() for line in self.get_input(__file__)]

        rules = list(map(parse_rule, puzzle_input))

        return can_contain("shiny gold", rules)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        rules = list(map(parse_rule_level, puzzle_input))

        return bag_levels("shiny gold", rules)
