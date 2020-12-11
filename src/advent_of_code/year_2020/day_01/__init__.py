from advent_of_code import Problem


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)
        numbers = sorted(int(num) for num in puzzle_input)

        goal = 2020

        for x_idx, x in enumerate(numbers):
            for y in numbers[x_idx:]:
                if x + y > goal:
                    break
                if x + y == goal:
                    return x * y

        raise Exception(f"Didn't find two entries that add up to {goal}")


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)
        numbers = sorted(int(num) for num in puzzle_input)

        goal = 2020

        for x_idx, x in enumerate(numbers):
            for y_idx, y in enumerate(numbers[x_idx:]):
                for z in numbers[y_idx:]:
                    if x + y + z > goal:
                        break
                    if x + y + z == goal:
                        return x * y * z

        raise Exception(f"Didn't find two entries that add up to {goal}")
