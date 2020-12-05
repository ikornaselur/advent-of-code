import argparse
import os
import pickle
from hashlib import sha1
from importlib import import_module
from typing import Dict, Iterator, List, Literal, Optional, Union

from rich import box
from rich.console import Console
from rich.live_render import LiveRender
from rich.table import Table

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
            raise Exception(f"Folder not found! ({path})")

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

    def __init__(self: "Target", year: int, day: int) -> None:
        self.year = year
        self.day = day

    def get_path(self: "Target") -> str:
        root = os.path.dirname(os.path.abspath(__file__))
        return os.path.join(root, f"year_{self.year}", f"day_{self.day}")

    def get_solution(self: "Target", part: Literal[1, 2]) -> Solution:
        day = import_module(f"advent_of_code.year_{self.year}.day_{self.day}")
        Part = getattr(day, f"Part{part}")

        return Part().get_solution()

    def __repr__(self: "Target") -> str:
        return f"<Target {self.year} day {self.day}>"


def discover(year: Optional[int] = None) -> Iterator[Target]:
    if year:
        year_range = [year]
    else:
        year_range = list(range(2015, 2021))

    for year in year_range:
        for day in range(1, 25):
            if not os.path.exists(f"./src/advent_of_code/year_{year}/day_{day}"):
                # Day not done yet, assume no further days this year
                break

            yield Target(year, day)


def run_basic(year: Optional[int] = None, run_day: Optional[int] = None) -> None:
    targets = discover(year)
    cache = Cache()

    for target in targets:
        path = target.get_path()
        path_hash = f"{cache.hash_folder(path)}"

        lst: List[Literal[1, 2]] = [1, 2]  # For mypy...
        for part in lst:
            part_hash = f"{path_hash}-part_{part}"
            if target.day == run_day or not (solution := cache.get_cached(part_hash)):
                solution = target.get_solution(part)
                cache.update_cache(part_hash, solution)

            print(f"{target.year} Day {target.day} - Part {part}: {solution}")

    cache.flush()


def run_complex(year: Optional[int] = None, run_day: Optional[int] = None) -> None:
    cache = Cache()
    console = Console()

    filter_year = int(os.environ.get("YEAR", 0))

    by_year: Dict[int, List[Target]] = {}
    for target in discover(year):
        if target.year not in by_year:
            by_year[target.year] = []
        by_year[target.year].append(target)

    for year, targets in by_year.items():
        if filter_year and filter_year != year:
            continue
        table = Table(
            show_header=True,
            box=box.HORIZONTALS,
            header_style="bold",
            show_edge=False,
            show_lines=False,
            title=f":christmas_tree: [b]Year {year} :christmas_tree:",
            title_style="white",
        )
        table.add_column("")
        table.add_column("Part 1")
        table.add_column("Part 2")

        # Set up rows for each day
        for day in range(1, 26):
            table.add_row(f"Day {day}", "", "")

        # Set up live render
        live_render = LiveRender(table)

        # Print initial table
        console.print(live_render)

        for target in targets:
            path = target.get_path()
            path_hash = f"{cache.hash_folder(path)}"

            lst: List[Literal[1, 2]] = [1, 2]  # For mypy...
            for part in lst:
                part_hash = f"{path_hash}-part_{part}"
                if target.day == run_day or not (solution := cache.get_cached(part_hash)):
                    table.columns[part]._cells[target.day - 1] = "..."
                    console.print(live_render.position_cursor(), live_render)
                    solution = target.get_solution(part)
                    cache.update_cache(part_hash, solution)

                table.columns[part]._cells[target.day - 1] = str(solution)
                console.print(live_render.position_cursor(), live_render)

    cache.flush()


def run() -> None:
    parser = argparse.ArgumentParser(description="Process all solutions for a given year")
    parser.add_argument("year", metavar="YEAR", type=int, help="The year to process")
    parser.add_argument(
        "-s",
        "--simple",
        default=False,
        action="store_true",
        help="Print out the results with just simple prints",
    )
    parser.add_argument(
        "-r",
        "--run-day",
        type=int,
        help="Force run a specific day, bypassing any caching",
    )
    args = parser.parse_args()

    if args.simple:
        run_basic(args.year, run_day=args.run_day)
    else:
        run_complex(args.year, run_day=args.run_day)
