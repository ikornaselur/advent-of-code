import math
from functools import reduce
from typing import Dict, List, Tuple

from advent_of_code import Problem


def egcd(a: int, b: int) -> Tuple[int, int, int]:
    """ Extended GCD """
    if a == 0:
        return (b, 0, 1)
    gcd, x, y = egcd(b % a, a)
    return (gcd, y - (b // a) * x, x)


def two_bus_departure(bus_1: Tuple[int, int], bus_2: Tuple[int, int]) -> Tuple[int, int]:
    """Find the departure time for two buses

    Takes in bus tuples of (bus id, departure delay)
    Returns a tuple of (earliest departure, how many minutes between valid departures)
    """
    if bus_1[0] > bus_2[0]:
        bus_1, bus_2 = bus_2, bus_1

    a = bus_1[0]
    b = bus_2[0] * -1
    c = bus_1[1] - bus_2[1]

    gcd, x_star, y_star = egcd(a, b)
    p = c // gcd
    x_0 = x_star * p

    t = 0
    while (x := x_0 + (b // gcd) * t) < 0:
        t -= 1

    next_x = x_0 + (b // gcd) * (t - 1)

    earliest_departure = x * a
    between_departures = (next_x - x) * a

    return (between_departures, earliest_departure - bus_1[1])


def find_earliest_bus(earliest_departure: int, buses: List[int]) -> Tuple[int, int]:
    next_departures: Dict[int, int] = {}  # Key: Departure, Value: Bus
    for bus in buses:
        next_departures[earliest_departure - (earliest_departure % bus) + bus] = bus

    next_departure = min(next_departures)
    return next_departures[next_departure], next_departure


def find_earliest_subsequent(buses_raw: str) -> int:
    """
    For buses a, b and c:
        1. Find the matching departure and cycle time for a and b
        2. Treat the output from a and b as a single bus x
        3. Find the matching departure and cycle time for x and c
    """
    buses = [(int(bus), idx) for idx, bus in enumerate(buses_raw.split(",")) if bus != "x"]

    out = two_bus_departure(buses[0], buses[1])

    for bus in buses[2:]:
        temp = (out[0], out[0] - out[1])
        out = two_bus_departure(temp, bus)

    return out[1]


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        earliest_departure = int(puzzle_input[0])
        buses = [int(bus) for bus in puzzle_input[1].split(",") if bus != "x"]

        bus, departure = find_earliest_bus(earliest_departure, buses)

        return bus * (departure - earliest_departure)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return find_earliest_subsequent(puzzle_input[1])
