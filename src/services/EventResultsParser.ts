import parse from 'csv-parse/lib/sync';
import {
  ClassCategoryResults,
  ClassResults,
  Driver,
  EventResults,
} from '../models';

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
    });

    const eventResults: EventResults = {};
    let classCategoryResults: ClassCategoryResults;
    let currentClass: ClassResults;
    rows.forEach((row) => {
      if (row[row.length - 1].endsWith('Category')) {
        // Just the class category
        classCategoryResults = {};
        eventResults[row[row.length - 1]] = classCategoryResults;
      } else if (row.length === 65) {
        // Class + table header + first row
        const className = row[11];
        currentClass = new ClassResults(className);
        classCategoryResults[className] = currentClass;
        EventResultsParser.processResultsRow(row.slice(15), currentClass);
      } else if (row[0] === 'Results') {
        EventResultsParser.processResultsRow(row.slice(11), currentClass);
      } else {
        // Class + table header + first row (when missing extra header prefix)
        const classname = row[0];
        currentClass = new ClassResults(classname);
        classCategoryResults[classname] = currentClass;
        EventResultsParser.processResultsRow(row.slice(4), currentClass);
      }
    });

    const categoriesToRemove: string[] = [];
    Object.entries(eventResults).forEach(([category, categoryResults]) => {
      const classesToRemove: string[] = [];
      Object.values(categoryResults).forEach((classResults) => {
        if (!classResults.drivers.length) {
          classesToRemove.push(classResults.carClass);
        }
      });
      classesToRemove.forEach((carClass) => delete categoryResults[carClass]);

      if (!Object.keys(categoryResults).length) {
        categoriesToRemove.push(category);
      }
    });
    categoriesToRemove.forEach((category) => delete eventResults[category]);

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
      const times = row.slice(this.HEADER.length, row.length - 4);

      const indexOfFastest = 3;
      const indexOfDifference = 13;
      const fastest = times[indexOfFastest];
      const difference = times[indexOfDifference];

      const fixedTimes = [
        ...times.slice(0, indexOfFastest),
        ...times.slice(indexOfFastest + 1, indexOfDifference),
        ...times.slice(indexOfDifference + 1),
      ].filter((t) => !!t.trim());

      if (meta[0] === 'T') {
        classResults.trophyCount += 1;
      }
      const driver = new Driver(classResults.carClass, meta, fixedTimes);
      if (driver.times.length) {
        classResults.drivers.push(driver);
      }

      return [...meta, ...fixedTimes, fastest, difference];
    }
  }
}
