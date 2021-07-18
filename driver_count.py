import csv
import typing


def read(filename: str) -> typing.List[str]:
    with open(filename, 'r') as e1:
        r1 = csv.reader(e1.readlines())
    return [r[1] for r in r1 if r[0] != 'Position' and r[-1] != '0']


def main():
    names1 = read('/home/david/event_pax_results.csv')
    names2 = read('/home/david/event_pax_results 2.csv')

    names = list(set(names1 + names2))

    print(f'Total: {len(names)}')
    [print(name) for name in sorted(names)]


main()
