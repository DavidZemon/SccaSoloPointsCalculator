#!/usr/bin/python3

import math
import sys


def main() -> None:
    print(calculate_class_points(float(sys.argv[1]), float(sys.argv[2])))


def calculate_class_points(fastest: float, actual: float) -> int:
    if fastest == actual:
        return 10000
    else:
        return round((fastest / actual) * 10_000)


if __name__ == '__main__':
    main()
