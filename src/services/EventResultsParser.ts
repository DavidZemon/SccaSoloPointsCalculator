import { parse } from 'csv-parse/lib/sync';
import * as rusty from 'rusty/rusty';
import {
  ClassResults,
  Driver,
  EventResults,
  ExportedDriver,
} from '../models';
import { calculateTrophies } from './utilities';

export class EventResultsParser {
  async parse(fileContents: string): Promise<EventResults> /*{
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
      .map((classResults) => {
        classResults.trophyCount = calculateTrophies(classResults.drivers);
        return classResults.drivers.sort((a, b) =>
          rusty.LapTime.compare(a.bestLap(), b.bestLap()),
        );
      })
      .forEach((drivers) => {
        drivers.forEach((driver, index) => (driver.position = index + 1));
      });

    return eventResults;
  }*/ {
    return Promise.resolve({});
  }
}
