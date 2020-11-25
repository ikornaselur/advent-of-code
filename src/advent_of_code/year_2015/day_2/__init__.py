from advent_of_code import Problem


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        boxes = self.get_input(__file__)
        total = 0
        for box in boxes:
            dimensions = sorted(int(dim) for dim in box.split("x"))

            # Area is 2*l*w + 2*w*h + 2*h*l, plus area of the smallest side,
            # which will be the lower two dimensions. By sorting the dimensions
            # we can just make area be 3*x*y + 2*y*z + 2*x*z assuming that x
            # and y are the shorter two dimensions
            x = dimensions[0]
            y = dimensions[1]
            z = dimensions[2]

            area = 3 * x * y + 2 * y * z + 2 * x * z

            total += area

        return total


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        pass
