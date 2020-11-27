from hashlib import md5

from advent_of_code import Problem


class Part1(Problem):
    def get_solution(self: "Part1", num_of_zeroes: int = 5) -> int:
        puzzle_input = self.get_input(__file__)[0].strip()

        zeroes = num_of_zeroes * "0"

        idx = 0
        digest = ""
        while not digest.startswith(zeroes):
            idx += 1
            digest = md5(bytes(f"{puzzle_input}{idx}", "utf-8")).hexdigest()

        return idx


class Part2(Part1):
    def get_solution(self: "Part2", num_of_zeroes: int = 6) -> int:
        return super().get_solution(num_of_zeroes=num_of_zeroes)
