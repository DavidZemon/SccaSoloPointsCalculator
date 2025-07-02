import { parse } from 'csv-parse/lib/sync';
import { Table } from 'react-bootstrap';
import { JSX } from 'react';

interface CsvTableProps {
  csv: string;
  keyBuilder: (row: string[], index: number) => string;
}

export function CsvTable({ csv, keyBuilder }: CsvTableProps): JSX.Element {
  const lines: string[][] = parse(csv, {
    columns: false,
    relax_column_count_less: true,
  });
  const [header, ...drivers] = lines;
  return (
    <Table striped hover borderless>
      <thead>
        <tr>
          {header.map((h) => (
            <th key={h}>{h}</th>
          ))}
        </tr>
      </thead>

      <tbody>
        {drivers.map((row, i) => {
          const rowKey = keyBuilder(row, i);
          return (
            <tr key={rowKey}>
              {row.map((column, i) => (
                <td
                  key={`${rowKey} - ${header[i]}`}
                  colSpan={row.length === 1 ? header.length : 1}
                >
                  {column}
                </td>
              ))}
            </tr>
          );
        })}
      </tbody>
    </Table>
  );
}
