import csv
import os

from typing import Dict


def get(mapping: Dict[str, str], name: str) -> str:
    try:
        return mapping[name.lower()]
    except KeyError:
        return ''


def main() -> None:
    with open('./SampleData/event_results.csv') as f:
        reader = csv.reader(f)
        results_lines = [line for line in reader]

    with open('./SampleData/20210625-2021ProntoEventExport.csv') as f:
        reader = csv.reader(f)
        msr_lines = [line for line in reader]

    mapping: Dict[str, str] = {}
    for line in msr_lines[1:]:
        mapping[f'{line[1].lower()} {line[0].lower()}'] = line[7]

    with open('fixed_full_results.csv', 'w') as f:
        for line in results_lines:
            if len(line) > 1 and line[1] and not line[1] == 'Name':
                line[-1] = get(mapping, line[1])

            partial_line = '","'.join(line)
            f.write(f'"{partial_line}"{os.linesep}')


main()
