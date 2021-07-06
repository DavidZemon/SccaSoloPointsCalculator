import parse from 'csv-parse/lib/sync';
import { ClassResults, Driver, EventResults, LapTime } from '../models';

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

    // Prune empty classes. This occurs when a class has at least one registered driver for the event, but no drivers in
    // that class end up driving
    const classesToRemove: string[] = [];
    Object.values(eventResults)
      .filter((classResults) => !classResults.drivers.length)
      .forEach((classResults) => {
        classesToRemove.push(classResults.carClass);
      });
    classesToRemove.forEach((carClass) => delete eventResults[carClass]);

    // Fix any sorting issues with drivers, such as a bug in Pronto that causes 1-day drivers in a 2-day event to place
    // higher than 2-day drivers
    Object.values(eventResults)
      .map((classResults) =>
        classResults.drivers.sort((a, b) =>
          LapTime.compare(a.bestLap(a.day2Times), b.bestLap(b.day2Times)),
        ),
      )
      .forEach((drivers) => {
        drivers.forEach((driver, index) => (driver.position = index + 1));
      });

    return eventResults;
  }

  static processResultsRow(row: string[], classResults?: ClassResults): void {
    if (!classResults)
      throw Error(
        `Class results object is uninitialized for row ${row.join(',')}`,
      );
    else {
      const meta = row.slice(0, this.HEADER.length);
      const times = row.slice(this.HEADER.length, row.length - 3);

      const day1Times = [times[0], times[3], times[5], times[1], times[2]];
      const day2Times = times.slice(12);

      if (meta[1] === 'T') {
        classResults.trophyCount += 1;
      }
      const driver = new Driver(
        classResults.carClass,
        meta,
        day1Times,
        day2Times,
      );
      if (driver.day1Times.length && driver.day2Times.length) {
        classResults.drivers.push(driver);
      }
    }
  }
}
