import parse from 'csv-parse/lib/sync';
import {
  CarClass,
  ClassResults,
  Driver,
  EventResults,
  ExportedDriver,
  LapTime,
} from '../models';

export class EventResultsParser {
  async parse(fileContents: string): Promise<EventResults> {
    await EventResultsParser.validateHeaderRow(fileContents);
    const header: string[] = parse(fileContents, {
      columns: false,
      skipEmptyLines: true,
      trim: true,
      toLine: 1,
    })[0];
    const drivers: ExportedDriver[] = parse(fileContents, {
      columns: true,
      trim: true,
      raw: true,
      relaxColumnCount: true,
      skipEmptyLines: true,
      cast: true,
      onRecord: (record) =>
        EventResultsParser.csvRecordInterceptor(header, record),
    });

    const eventResults: EventResults = {};
    drivers.forEach((exportedDriver) => {
      const driver = new Driver(exportedDriver);
      // If the driver has any times or if there was an error processing the driver, include the driver in the
      // results. The error will be presented to the user later
      if (
        driver.getTimes('day1')?.length ||
        driver.getTimes('day2')?.length ||
        driver.error
      ) {
        let currentClass = eventResults[exportedDriver.Class];
        if (!currentClass) {
          currentClass = new ClassResults(exportedDriver.Class);
          eventResults[exportedDriver.Class] = currentClass;
        }
        currentClass.drivers.push(driver);
      } else {
        console.warn(
          `Removing driver due to no times: ${exportedDriver.Number} ${exportedDriver.Class}`,
        );
      }
    });

    // Prune empty classes. This occurs when a class has at least one registered driver for the event, but no drivers in
    // that class end up driving
    const classesToRemove: CarClass[] = [];
    const emptyClassResults = Object.values(eventResults).filter(
      (classResults) => !classResults.drivers.length,
    );
    emptyClassResults.forEach((classResults) => {
      classesToRemove.push(classResults.carClass);
    });
    classesToRemove.forEach((carClass) => delete eventResults[carClass.short]);

    // Fix any sorting issues with drivers, such as a bug in Pronto that causes 1-day drivers in a 2-day event to place
    // higher than 2-day drivers
    Object.values(eventResults)
      .map((classResults) =>
        classResults.drivers.sort((a, b) =>
          LapTime.compare(a.bestLap(), b.bestLap()),
        ),
      )
      .forEach((drivers) => {
        drivers.forEach((driver, index) => (driver.position = index + 1));
      });

    return eventResults;
  }

  private static csvRecordInterceptor(
    header: string[],
    {
      raw,
      record: driver,
    }: {
      raw: string;
      record: ExportedDriver;
    },
  ): ExportedDriver {
    if (driver['Runs Day1']) driver.day1 = [];
    if (driver['Runs Day2']) driver.day2 = [];

    const firstTimeColumnHeader = 'Runs (Time/Cones/Penalty)';
    const firstTimeColumnIndex = header.indexOf(firstTimeColumnHeader);

    if (-1 === firstTimeColumnIndex) {
      throw new Error(
        `Missing critical column header: ${firstTimeColumnHeader}`,
      );
    } else {
      const verboseTimes = parse(raw.trim(), {
        columns: false,
        trim: true,
      })[0].slice(firstTimeColumnIndex);

      // Insert day 1 times
      for (
        let lapNumber = 0;
        lapNumber < (driver['Runs Day1'] || 0);
        lapNumber++
      ) {
        const firstIndex = 3 * lapNumber;
        const rawTime = parseFloat(verboseTimes[firstIndex]);
        const cones = parseInt(verboseTimes[firstIndex + 1]);
        const penalty = verboseTimes[firstIndex + 2] || undefined;
        driver.day1!.push(new LapTime(rawTime, cones, penalty));
      }

      // Insert day 2 times
      for (
        let lapNumber = 0;
        lapNumber < (driver['Runs Day2'] || 0);
        lapNumber++
      ) {
        const firstIndex = 3 * lapNumber + 3 * (driver['Runs Day1'] || 0);
        driver.day2!.push(
          new LapTime(
            parseFloat(verboseTimes[firstIndex]),
            parseInt(verboseTimes[firstIndex + 1]),
            verboseTimes[firstIndex + 2] || undefined,
          ),
        );
      }

      return driver;
    }
  }

  private static async validateHeaderRow(content: string) {
    const EXPECTED_HEADER =
      'Class, Class Category, Class Name, Number, First Name,Last Name, Car Year, Car Make, Car Model, Car Color, Member #, Rookie, Ladies, DSQ, Region, Best Run, Pax Index, Pax Time';
    if (!content.includes(EXPECTED_HEADER)) {
      throw new Error(
        `Expected results file to start with header: ${EXPECTED_HEADER}`,
      );
    }
  }
}
