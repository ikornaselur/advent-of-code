import os
from importlib import import_module
from typing import Iterator, Union

import click


class Target:
    year: int
    day: int
    part: int

    def __init__(self: "Target", year: int, day: int, part: int) -> None:
        self.year = year
        self.day = day
        self.part = part

    def get_solution(self: "Target") -> Union[str, int]:
        day = import_module(f"advent_of_code.year_{self.year}.day_{self.day}")
        Part = getattr(day, f"Part{self.part}")

        return Part().get_solution()

    def pretty_print(self: "Target") -> None:
        year = click.style(str(self.year), fg="red")
        day = click.style(f"Dec {self.day}", fg="yellow")
        part = click.style(f"Part {self.part}", fg="yellow")

        solution = click.style(str(self.get_solution()), fg="green")

        click.echo(f"[{year} {day} - {part}] {solution}")


def discover() -> Iterator[Target]:
    for year in range(2015, 2021):
        for day in range(1, 25):
            if not os.path.exists(f"./src/advent_of_code/year_{year}/day_{day}"):
                # Day not done yet, assume no further days this year
                break

            # Assume both parts implemented
            yield Target(year, day, 1)
            yield Target(year, day, 2)


def run() -> None:
    targets = discover()
    for target in targets:
        target.pretty_print()


if __name__ == "__main__":
    run()
