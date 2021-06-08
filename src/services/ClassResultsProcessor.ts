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

      this.appendIndividualResults(meta, fixedTimes, classResults);

      return [...meta, ...fixedTimes, fastest, difference];
    }
  }

  static appendIndividualResults(
    meta: string[],
    times: string[],
    classResults: ClassResults,
  ): void {
    if (meta[0] === 'T') {
      classResults.trophyCount += 1;
    }
    const individualResults: IndividualResults = {
      id: meta[4], // FIXME
      carClass: classResults.carClass,
      trophy: meta[0] === 'T',
      rookie: meta[1] === 'R',
      position: parseFloat(meta[2]),
      carNumber: parseInt(meta[3]),
      name: meta[4],
      carDescription: meta[5],
      times: times
        .filter((lapTime) => !!lapTime.trim())
        .map((lapTime) => new LapTime(lapTime)),
    };
    if (individualResults.times.length) {
      classResults.results.push(individualResults);
    }
  }
}
