import argparse
import csv
import os
from typing import List, Dict

HEADER = [
    "TR",
    "RK",
    "Pos",
    "Nbr",
    "Driver's name, Town",
    "Car, Sponsor",
    "Tire Mfg",
    "Rgn,Div",
    "Course 1",
    "Score"
]


class LapTime:
    time: float
    cones: int
    dnf: bool

    def __cmp__(self, other):
        pass  # TODO


class IndividualResults:
    id: str
    name: str
    car_number: int
    car_class: str
    times: List[LapTime]
    trophy: bool
    rookie: bool
    position: int


class ClassResults:
    trophy_count: int
    results: List[IndividualResults]


EventResults = Dict[str, ClassResults]


def main() -> None:
    args = parse_args()
    input_file = args.file
    output_file = args.output

    with open(input_file, 'rb') as f:
        raw_text = f.read(10000000)

    no_cr = raw_text.replace('\x0d'.encode(), ''.encode()).decode()
    lines = no_cr.splitlines(keepends=False)

    reader = csv.reader(lines, quotechar='"')
    sep_lines = []
    for row in reader:
        if row[-1].endswith('Category'):
            # Just the class category
            sep_lines.append([row[-1]])
        elif len(row) == 65:
            # Class + table header + first row
            sep_lines.append(row[11:15])
            sep_lines.append(HEADER)

            first_half = row[15:27]
            second_half = row[33:-4]
            sep_lines.append(move_fastest_time(first_half + second_half))
        elif row[0] == 'Results':
            first_half = row[11:23]
            second_half = row[29:-4]
            sep_lines.append(move_fastest_time(first_half + second_half))
        else:
            # Class + table header + first row (when missing extra header prefix)
            sep_lines.append(row[0:4])
            sep_lines.append(HEADER)
            sep_lines.append(move_fastest_time(row[4:16] + row[22:-4]))

    with open(output_file, 'w') as f:
        [f.write('"' + '","'.join(line[:19]) + f'"{os.linesep}') for line in sep_lines]


def move_fastest_time(row: List[str]) -> List[str]:
    index_of_fastest = 11
    index_of_difference = 15
    fastest = row[index_of_fastest]
    difference = row[index_of_difference]

    results = row[:index_of_fastest] + row[index_of_fastest + 1:index_of_difference] + row[index_of_difference + 1:]

    results[17] = fastest
    results[18] = difference
    return results


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()

    parser.add_argument('-f', '--file', help='File to process', default='full_results.csv')
    parser.add_argument('-o', '--output', help='Output file path', default='full_results.fixed.csv')

    return parser.parse_args()


if '__main__' == __name__:
    main()
