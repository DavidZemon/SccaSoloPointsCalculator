import { Component, ComponentPropsWithoutRef } from 'react';
import { parse } from 'csv-parse/lib/sync';
import { Table } from 'react-bootstrap';

interface CsvTableProps extends ComponentPropsWithoutRef<any> {
  csv: string;
  keyBuilder: (row: string[]) => string;
}

export class CsvTable extends Component<CsvTableProps> {
  public render(): JSX.Element {
    const lines: string[][] = parse(this.props.csv, {
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
          {drivers.map((row) => {
            const rowKey = this.props.keyBuilder(row);
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
}
