#!/usr/bin/python3

import argparse
import json
import csv

from typing import List, Dict


def main() -> None:
    args = parse_args()

    in_path = args.input
    out_path = args.output

    with open(in_path, 'r') as f:
        csv_in = csv.DictReader(f)

        fixed_rows: List[Dict[str, str]] = [convert(r) for r in csv_in]

    with open(out_path, 'w') as f:
        writer = csv.DictWriter(f, fixed_rows[0].keys(), quoting=csv.QUOTE_ALL)
        writer.writeheader()
        writer.writerows(fixed_rows)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()

    parser.add_argument('-i', '--input', required=True, help='Input file')
    parser.add_argument('-o', '--output', required=True, help='Output file')

    return parser.parse_args()


def convert(i: Dict[str, str]) -> Dict[str, str]:
    out = i.copy()

    out['Are you a novice?'] = '1' if out['Novice'] == '1' else '0'
    del out['Novice']

    out['Modifier/PAX'] = 'L' if out['Ladies'] == '1' else ''
    del out['Ladies']

    if 'Medical condition? (Optional)' in out.keys():
        del out['Medical condition? (Optional)']

    return out


if __name__ == '__main__':
    main()
