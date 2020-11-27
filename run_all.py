import os
import pickle
from hashlib import sha1
from importlib import import_module
from typing import Dict, Iterator, Optional, Union

import click

CACHE_FILE = "./.cache.pkl"

Solution = Union[int, str]


class Cache:
    __cache: Dict[str, Solution]
    auto_flush: bool

    def __init__(self: "Cache", auto_flush: bool = True) -> None:
        self.auto_flush = auto_flush
        if os.path.exists(CACHE_FILE):
            with open(CACHE_FILE, "rb") as f:
                self.__cache = pickle.load(f)
        else:
            self.__cache = {}

    def update_cache(self: "Cache", key: str, value: Solution) -> None:
        self.__cache[key] = value

        if self.auto_flush:
            self.flush()

    def get_cached(self: "Cache", key: str) -> Optional[Solution]:
        return self.__cache.get(key)

    def flush(self: "Cache") -> None:
        with open(CACHE_FILE, "wb") as f:
            pickle.dump(self.__cache, f)

    def hash_folder(self: "Cache", path: str) -> str:
        """Go through the files in the folder and produce a SHA1 hash of the contents"""
        if not os.path.exists(path):
            raise Exception("Folder not found!")

        folder_hash = sha1()

        for root, _, files in os.walk(path):
            for file in files:
                with open(os.path.join(root, file), "rb") as f:
                    for chunk in iter(lambda: f.read(4096), b""):
                        folder_hash.update(chunk)

        return folder_hash.hexdigest()


class Target:
    year: int
    day: int
    part: int

    def __init__(self: "Target", year: int, day: int, part: int) -> None:
        self.year = year
        self.day = day
        self.part = part

    def get_path(self: "Target") -> str:
        root = os.path.dirname(os.path.abspath(__file__))
        return os.path.join(root, "src/advent_of_code", f"year_{self.year}", f"day_{self.day}")

    def get_solution(self: "Target") -> Solution:
        day = import_module(f"advent_of_code.year_{self.year}.day_{self.day}")
        Part = getattr(day, f"Part{self.part}")

        return Part().get_solution()

    def __repr__(self: "Target") -> str:
        return f"<Target {self.year}-{self.day}. Part {self.part}>"


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
    cache = Cache()
    for target in targets:
        path = target.get_path()
        path_hash = f"{cache.hash_folder(path)}-part_{target.part}"

        if not (solution := cache.get_cached(path_hash)):
            print("Not cached")
            solution = target.get_solution()
            cache.update_cache(path_hash, solution)

        year = click.style(str(target.year), fg="red")
        day = click.style(f"Dec {target.day}", fg="yellow")
        part = click.style(f"Part {target.part}", fg="yellow")

        solution = click.style(str(solution), fg="green")

        click.echo(f"[{year} {day} - {part}] {solution}")
    cache.flush()


if __name__ == "__main__":
    run()
