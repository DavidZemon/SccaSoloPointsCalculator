import argparse
import csv
import os
from typing import List, Dict, Optional

import jsonpickle

HEADER = [
    "TR",
    "RK",
    "Pos",
    "Nbr",
    "Driver's name, Town",
    "Car, Sponsor",
    "Tire Mfg",
    "Rgn,Div",
]


class LapTime:
    time: Optional[float]
    cones: int
    dnf: bool
    rerun: bool

    def __init__(self, lap_time_str: str) -> None:
        super().__init__()
        if lap_time_str == 'DNF':
            self.dnf = True
            self.rerun = False
            self.time = None
            self.cones = 0
        elif lap_time_str == 'RRN':
            self.dnf = False
            self.rerun = True
            self.time = None
            self.cones = 0
        else:
            self.dnf = False
            time_parts = lap_time_str.split('(')
            self.time = float(time_parts[0])
            self.cones = 0 if len(time_parts) == 1 else int(time_parts[1][:-1])

    def __lt__(self, other) -> bool:
        if self.time is None:
            return False
        else:
            return self.time < other.time

    def __gt__(self, other) -> bool:
        return not self.__lt__(other)


class IndividualResults:
    id: str
    name: str
    car_number: int
    car_class: str
    car_description: str
    times: List[LapTime]
    trophy: bool
    rookie: bool
    position: int


class ClassResults:
    trophy_count: int
    car_class: str
    results: List[IndividualResults]

    def __init__(self, car_class: str) -> None:
        super().__init__()
        self.trophy_count = 0
        self.car_class = car_class
        self.results = []


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
    final_lines = []
    results: EventResults = {}
    current_class: Optional[ClassResults] = None
    for row in reader:
        row = [word.strip() for word in row]
        if row[-1].endswith('Category'):
            # Just the class category
            final_lines.append([row[-1]])
        elif len(row) == 65:
            # Class + table header + first row
            class_header = row[11:15]
            current_class = ClassResults(class_header[0])
            results[class_header[0]] = current_class
            final_lines.append(class_header)

            final_lines.append(HEADER)

            final_lines.append(process_results_row(current_class, row[15:]))
        elif row[0] == 'Results':
            final_lines.append(process_results_row(current_class, row[11:]))
        else:
            # Class + table header + first row (when missing extra header prefix)
            class_header = row[0:4]
            current_class = ClassResults(class_header[0])
            results[class_header[0]] = current_class
            final_lines.append(class_header)

            final_lines.append(HEADER)
            final_lines.append(process_results_row(current_class, row[4:]))

    print(jsonpickle.encode(results))

    with open(output_file, 'w') as f:
        [f.write('"' + '","'.join(line) + f'"{os.linesep}') for line in final_lines]


def process_results_row(class_results: Optional[ClassResults], row: List[str]) -> List[str]:
    if class_results is None:
        raise Exception(f'Class results object is uninitialized for row {",".join(row)}')
    else:
        meta = row[:len(HEADER)]
        times = row[len(HEADER):]

        index_of_fastest = 3
        index_of_difference = 13
        fastest = times[index_of_fastest]
        difference = times[index_of_difference]

        fixed_times = times[:index_of_fastest] + \
                      times[index_of_fastest + 1:index_of_difference] + \
                      times[index_of_difference + 1:-4]
        fixed_times = [data for data in fixed_times if data]
        fixed_times += [''] * (12 - len(fixed_times))

        fixed_times[-2] = fastest
        fixed_times[-1] = difference

        fixed_row = meta + fixed_times
        append_individual_results(class_results, fixed_row)

        return fixed_row


def append_individual_results(class_results: ClassResults, fixed_row: List[str]) -> None:
    if fixed_row[0] == 'T':
        class_results.trophy_count += 1
    individual_results = IndividualResults()
    individual_results.car_class = class_results.car_class
    individual_results.trophy = fixed_row[0] == 'T'
    individual_results.rookie = fixed_row[1] == 'R'
    individual_results.position = int(float(fixed_row[2]))
    individual_results.car_number = int(fixed_row[3])
    individual_results.name = fixed_row[4]
    individual_results.car_description = fixed_row[5]
    individual_results.times = [LapTime(lap_time) for lap_time in fixed_row[6:18] if lap_time.strip()]
    if len(individual_results.times):
        class_results.results.append(individual_results)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()

    parser.add_argument('-f', '--file', help='File to process', default='full_results.csv')
    parser.add_argument('-o', '--output', help='Output file path', default='full_results.fixed.csv')

    return parser.parse_args()


if '__main__' == __name__:
    main()
