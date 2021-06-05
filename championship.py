#!/usr/bin/python3

import math
import sys
from typing import *


def main() -> None:
    print(calculate_class_points(float(sys.argv[1]), float(sys.argv[2])))


def calculate_class_points(fastest: float, actual: float) -> int:
    if fastest == actual:
        return 10000
    else:
        return round((fastest / actual) * 10_000)


def calculate_championship_points(points: List[Optional[int]]) -> int:
    event_count = len(points)
    points = [0 if event_points is None else event_points for event_points in points]
    if event_count < 4:
        return sum([0 if event_points is None else event_points for event_points in points])
    else:
        events_to_count = round(event_count / 2) + 2
        sorted_points = sorted(points, reverse=True)
        return sum(sorted_points[:events_to_count])


if __name__ == '__main__':
    main()
