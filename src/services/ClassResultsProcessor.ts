import parse from 'csv-parse/lib/sync';
import {
  ClassCategoryResults,
  ClassResults,
  EventResults,
  IndividualResults,
  LapTime,
} from '../models';

export class ClassResultsProcessor {
  public static readonly HEADER = [
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
    const rows: string[][] = parse(fileContents, {
      columns: false,
      ltrim: true,
      rtrim: true,
      relaxColumnCount: true,
    });

    const finalLines = [];
    const eventResults: EventResults = {};
    let classCategoryResults: ClassCategoryResults;
    let currentClass: ClassResults;
    rows.forEach((row) => {
      if (row[row.length - 1].endsWith('Category')) {
        // Just the class category
        const categoryName = row[row.length - 1];
        classCategoryResults = {};
        eventResults[categoryName] = classCategoryResults;
        finalLines.push([categoryName]);
      } else if (row.length === 65) {
        // Class + table header + first row
        const classHeader = row.slice(11, 15);
        const className = classHeader[0];
        currentClass = new ClassResults(className);
        classCategoryResults[className] = currentClass;
        finalLines.push(classHeader);
        finalLines.push(ClassResultsProcessor.HEADER);
        finalLines.push(
          ClassResultsProcessor.processResultsRow(row.slice(15), currentClass),
        );
      } else if (row[0] === 'Results') {
        finalLines.push(
          ClassResultsProcessor.processResultsRow(row.slice(11), currentClass),
        );
      } else {
        // Class + table header + first row (when missing extra header prefix)
        const classHeader = row.slice(0, 4);
        const classname = classHeader[0];
        currentClass = new ClassResults(classname);
        classCategoryResults[classname] = currentClass;
        finalLines.push(classHeader);

        finalLines.push(ClassResultsProcessor.HEADER);
        finalLines.push(
          ClassResultsProcessor.processResultsRow(row.slice(4), currentClass),
        );
      }
    });
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
      const times = row.slice(this.HEADER.length);

      const indexOfFastest = 3;
      const indexOfDifference = 13;
      const fastest = times[indexOfFastest];
      const difference = times[indexOfDifference];

      const fixedTimes = times
        .slice(0, indexOfFastest)
        .concat(times.slice(indexOfFastest + 1, indexOfDifference))
        .concat(times.slice(indexOfDifference + 1, times.length - 4))
        .filter((t) => !!t);

      fixedTimes[20] = fastest;
      fixedTimes[21] = difference;

      const fixedRow = meta.concat(fixedTimes);
      this.appendIndividualResults(fixedRow, classResults);

      return fixedRow;
    }
  }

  static appendIndividualResults(
    fixedRow: string[],
    classResults: ClassResults,
  ): void {
    if (fixedRow[0] === 'T') {
      classResults.trophyCount += 1;
    }
    const individualResults: IndividualResults = {
      id: fixedRow[4], // FIXME
      carClass: classResults.carClass,
      trophy: fixedRow[0] === 'T',
      rookie: fixedRow[1] === 'R',
      position: parseFloat(fixedRow[2]),
      carNumber: parseInt(fixedRow[3]),
      name: fixedRow[4],
      carDescription: fixedRow[5],
      times: fixedRow
        .slice(8, 20)
        .filter((lapTime) => lapTime.trim())
        .map((lapTime) => new LapTime(lapTime)),
    };
    if (individualResults.times.length) {
      classResults.results.push(individualResults);
    }
  }
}
