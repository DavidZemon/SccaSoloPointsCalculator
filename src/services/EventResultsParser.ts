import parse from 'csv-parse/lib/sync';
import { ClassResults, Driver, EventResults } from '../models';

export class EventResultsParser {
  public static readonly HEADER = [
    'TR',
    'RK',
    'Pos',
    'Nbr',
    "Driver's name, Town",
    'Car, Sponsor',
    'Tire Mfg',
    'Rgn,Div',
  ];

  async parse(fileContents: string): Promise<EventResults> {
    const rows: string[][] = parse(fileContents, {
      columns: false,
      ltrim: true,
      rtrim: true,
      relaxColumnCount: true,
      skipEmptyLines: true,
    });

    const eventResults: EventResults = {};
    let currentClass: ClassResults;
    rows
      // Filter out any rows with only empty cells
      .filter((row) => row.filter((cell) => cell.trim().length).length)
      .forEach((row) => {
        if (row.length === 61) {
          // Class + table header + first row
          const className = row[10];
          currentClass = new ClassResults(className);
          eventResults[className] = currentClass;
          EventResultsParser.processResultsRow(row.slice(14), currentClass);
        } else if (row[0] === 'Results') {
          EventResultsParser.processResultsRow(row.slice(10), currentClass);
        } else {
          // Class + table header + first row (when missing extra header prefix)
          const classname = row[0];
          currentClass = new ClassResults(classname);
          eventResults[classname] = currentClass;
          EventResultsParser.processResultsRow(row.slice(4), currentClass);
        }
      });

    const classesToRemove: string[] = [];
    Object.values(eventResults)
      .filter((classResults) => !classResults.drivers.length)
      .forEach((classResults) => {
        classesToRemove.push(classResults.carClass);
      });
    classesToRemove.forEach((carClass) => delete eventResults[carClass]);

    return eventResults;
  }

  static processResultsRow(
    row: string[],
    classResults?: ClassResults,
  ): string[] {
    if (!classResults)
      throw Error(
        `Class results object is uninitialized for row ${row.join(',')}`,
      );
    else {
      const meta = row.slice(0, this.HEADER.length);
      const times = row.slice(this.HEADER.length, row.length - 3);

      const indexOfFastest = 2;
      const indexOfDifference = 11;
      const fastest = times[indexOfFastest];
      const difference = times[indexOfDifference];

      const fixedTimes = [times[0], times[3], times[5], times[1], times[2]];

      if (meta[1] === 'T') {
        classResults.trophyCount += 1;
      }
      const driver = new Driver(
        classResults.carClass,
        meta,
        fixedTimes,
        fastest,
      );
      if (driver.times.length) {
        classResults.drivers.push(driver);
      }

      return [...meta, ...fixedTimes, fastest, difference];
    }
  }
}
