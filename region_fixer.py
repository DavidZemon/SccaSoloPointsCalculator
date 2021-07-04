import csv
from typing import Dict


def main() -> None:
    with open('/home/david/20210625-2021ProntoEventExport.csv', 'r') as f:
        with_region_lines = csv.reader(f.readlines()[1:])

    with open('/home/david/event_results.csv', 'r') as f:
        event_result_lines = csv.reader(f.readlines())

    region_mapping: Dict[str, str] = {}
    for row in with_region_lines:
        region_mapping[f'{row[1]} {row[0]}'] = row[7]

    with open('/home/david/event_results_with_region.csv', 'w') as f:
        writer = csv.writer(f)
        for row in event_result_lines:
            if row[0] in ['', 'T']:
                writer.writerow(row[:6] + [region_mapping[row[4]]] + row[7:])
            else:
                writer.writerow(row)


main()
