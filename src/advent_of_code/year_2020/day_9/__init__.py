from typing import List

from advent_of_code import Problem


def get_first_invalid(feed: List[int], preamble: int) -> int:
    queue = feed[:preamble]

    for pointer in range(preamble, len(feed)):
        next_value = feed[pointer]
        if not validate(next_value, queue):
            return next_value

        queue = queue[1:] + [next_value]

    raise Exception("Unable to find invalid value")


def validate(value: int, queue: List[int]) -> bool:
    sorted_queue = sorted(queue)

    for idx, x in enumerate(sorted_queue):
        for y in sorted_queue[idx:]:
            if x + y == value:
                return True

    return False


def get_contiguous_set(target: int, feed: List[int]) -> List[int]:
    start = 0
    end = 1

    while (current_sum := sum(feed[start : end + 1])) != target:
        if current_sum < target:
            end += 1
        elif current_sum > target:
            start += 1

    return feed[start : end + 1]


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = [int(line) for line in self.get_input(__file__)]

        return get_first_invalid(puzzle_input, 25)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = [int(line) for line in self.get_input(__file__)]

        first_invalid = get_first_invalid(puzzle_input, 25)

        contiguous_set = get_contiguous_set(first_invalid, puzzle_input)

        return min(contiguous_set) + max(contiguous_set)
