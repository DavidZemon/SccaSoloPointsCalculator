import parse from 'csv-parse/lib/sync';
import { EventResults } from '../models';

export class ClassResultsProcessor {
  private static readonly HEADER = [
    'TR',
    'RK',
    'Pos',
    'Nbr',
    "Driver's name, Town",
    'Car, Sponsor',
    'Tire Mfg',
    'Rgn,Div',
    'Course 1',
    'Score',
  ];

  async process(fileContents: string): Promise<EventResults> {
    const lines: string[][] = parse(fileContents, {
      columns: false,
      ltrim: true,
      rtrim: true,
      relaxColumnCount: true,
    });

    lines.forEach((line) => {
      console.log(`Line: ${JSON.stringify(line)}`);
    });

    const eventResults = {};

    return eventResults;
  }
}
