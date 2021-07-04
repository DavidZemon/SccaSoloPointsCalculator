import { read as xlsxRead, Sheet, utils as xlsxUtils } from 'xlsx';
import { toast } from 'react-toastify';
import parse from 'csv-parse/lib/sync';
import {
  ChampionshipDriver,
  ChampionshipResults,
  ChampionshipType,
  ClassChampionshipDriver,
  ClassChampionshipResults,
  Driver,
  EventResults,
  IndexedChampionshipResults,
  IndexedChampionshipType,
} from '../models';
import { PaxService } from './PaxService';
import { calculatePointsForDriver } from './utilities';

export class ChampionshipResultsParser {
  constructor(private readonly paxService: PaxService) {}

  async parse(
    inputFiles: Record<ChampionshipType, File | undefined>,
    eventResults: EventResults,
    newLadies: string[],
  ): Promise<ChampionshipResults> {
    const allDriversForEvent = Object.values(eventResults)
      .filter((classResults) => classResults.carClass !== 'Fun Class')
      .map((classResults) => classResults.drivers)
      .flat()
      .filter((driver) => !driver.dsq);
    const driversById = allDriversForEvent.reduce((o, driver) => {
      o[driver.id] = driver;
      return o;
    }, {} as Record<string, Driver>);

    const results: ChampionshipResults = {};
    await Promise.all(
      Object.entries(inputFiles)
        .filter(([_, f]) => !!f)
        .map(async ([championshipType, f]) => {
          const workBook = xlsxRead(await f!.arrayBuffer(), { type: 'buffer' });
          try {
            const rows: string[][] = ChampionshipResultsParser.parseXlsx(
              f?.name,
              workBook.SheetNames,
              workBook.Sheets,
            );
            switch (championshipType as ChampionshipType) {
              case 'Class':
                results['Class'] = this.parseClassResults(rows, eventResults);
                break;
              case 'PAX':
                const bestPaxTimeOfDay = Math.min(
                  ...allDriversForEvent.map(
                    (driver) =>
                      (driver.combined.time || Infinity) *
                      this.paxService.getMultiplierFromLongName(
                        driver.carClass,
                      ),
                  ),
                );
                results[championshipType as IndexedChampionshipType] =
                  this.parseIndexedResults(rows, driversById, bestPaxTimeOfDay);
                break;
              case 'Novice':
                const novices = allDriversForEvent.filter(
                  (driver) => driver.rookie,
                );
                const fastestNoviceOfDay = Math.min(
                  ...novices.map(
                    (driver) =>
                      (driver.combined.time || Infinity) *
                      this.paxService.getMultiplierFromLongName(
                        driver.carClass,
                      ),
                  ),
                );
                results[championshipType as IndexedChampionshipType] =
                  this.parseIndexedResults(
                    rows,
                    novices.reduce((o, d) => {
                      o[d.id] = d;
                      return o;
                    }, {} as Record<string, Driver>),
                    fastestNoviceOfDay,
                  );
                break;
              case 'Ladies':
                const ladiesNames = [
                  ...rows.slice(3).map((row) => row[1].toLowerCase()),
                  ...newLadies.map((name) => name.toLowerCase()),
                ];
                const ladies = allDriversForEvent.filter((driver) =>
                  ladiesNames.includes(driver.name.toLowerCase()),
                );
                const fastestLadiesOfDay = Math.min(
                  ...ladies.map(
                    (driver) =>
                      (driver.combined.time || Infinity) *
                      this.paxService.getMultiplierFromLongName(
                        driver.carClass,
                      ),
                  ),
                );
                results[championshipType as IndexedChampionshipType] =
                  this.parseIndexedResults(
                    rows,
                    ladies.reduce((o, d) => {
                      o[d.id] = d;
                      return o;
                    }, {} as Record<string, Driver>),
                    fastestLadiesOfDay,
                  );
                break;
            }
          } catch (e) {
            console.error(e);
            toast.error(e.message ? e.message : e.toString());
          }
        }),
    );
    return results;
  }

  private parseClassResults(
    rows: string[][],
    eventResults: EventResults,
  ): ClassChampionshipResults {
    // Two header rows (rank + driver), plus two total rows (points + "Best N of M")
    const pastEventCount = rows[5].length - 4;

    // Start by grouping rows by class
    const rowsByClassAndDriverId: Record<
      string,
      Record<string, Omit<ClassChampionshipDriver, 'totalPoints'>>
    > = {};
    let classRows: Record<string, Omit<ClassChampionshipDriver, 'totalPoints'>>;
    let currentClass: string;
    rows.slice(4).forEach((row) => {
      // If the first cell is non-numeric, it is a class header
      if (isNaN(parseInt(row[0]))) {
        currentClass = row[0].split(' - ')[1];
        if (!currentClass) {
          currentClass = row[0].split(' – ')[1];
        }
        rowsByClassAndDriverId[currentClass] = classRows = {};
      } else {
        const id = row[1].toLowerCase().trim();
        classRows[id] = {
          carClass: currentClass,
          id,
          name: row[1],
          points: row.slice(2, row.length - 2).map((p) => parseInt(p)),
        };
      }
    });

    const newEventDriversByClassAndId: Record<
      string,
      Record<string, Driver>
    > = {};

    // Get this event's driver IDs
    Object.values(eventResults)
      .filter((classResults) => classResults.carClass !== 'Fun Class')
      .forEach((classResults) => {
        newEventDriversByClassAndId[classResults.carClass] =
          classResults.drivers
            .filter((driver) => !driver.dsq)
            .reduce((o, d) => {
              o[d.id] = d;
              return o;
            }, {} as Record<string, Driver>);
      });

    const allDriverIdsByClass: Record<string, string[]> = {};
    [
      ...new Set([
        ...Object.keys(rowsByClassAndDriverId),
        ...Object.keys(newEventDriversByClassAndId),
      ]),
    ].forEach((carClass) => {
      allDriverIdsByClass[carClass] = [
        ...new Set([
          ...Object.keys(rowsByClassAndDriverId[carClass] || []),
          ...Object.keys(newEventDriversByClassAndId[carClass] || []),
        ]),
      ];
    });

    const driversByClass: Record<string, ChampionshipDriver[]> = {};
    Object.entries(allDriverIdsByClass).forEach(([carClass, driverIds]) => {
      const classHistory = rowsByClassAndDriverId[carClass] || [];
      const newEventDriversById = newEventDriversByClassAndId[carClass] || [];

      const bestTimeOfDay =
        Math.min(
          ...(Object.values(newEventDriversById)
            .map((driver) => driver.combined.time)
            .filter((t) => t) as number[]),
        ) * this.paxService.getMultiplierFromLongName(carClass);
      driversByClass[carClass] = driverIds.map(
        (driverId): ClassChampionshipDriver => ({
          ...this.buildDriver(
            driverId,
            classHistory,
            newEventDriversById,
            bestTimeOfDay,
            pastEventCount,
          ),
          carClass,
        }),
      );
    });

    return {
      organization: rows[0][0].trim(),
      year: parseInt(rows[1][0].split(' ')[0]),
      driversByClass: driversByClass,
    };
  }

  private parseIndexedResults(
    rows: string[][],
    driversForEventById: Record<string, Driver>,
    bestIndexTimeOfDay: number,
  ): IndexedChampionshipResults {
    const previousDrivers = rows
      .slice(3)
      .map((row): ChampionshipDriver => {
        const allPoints = row.slice(2, row.length - 2).map((p) => parseInt(p));
        return {
          id: row[1].toLowerCase().trim(),
          name: row[1],
          points: allPoints,
          totalPoints: ChampionshipResultsParser.sumPoints(allPoints),
        };
      })
      .reduce((o, d) => {
        o[d.id] = d;
        return o;
      }, {} as Record<string, ChampionshipDriver>);

    const pastEventCount = Object.values(previousDrivers)[0].points.length;
    return {
      organization: rows[0][0],
      year: parseInt(rows[1][0].split(' ')[0]),
      drivers: [
        ...new Set([
          ...Object.keys(driversForEventById),
          ...Object.keys(previousDrivers),
        ]),
      ].map((driverId) =>
        this.buildDriver(
          driverId,
          previousDrivers,
          driversForEventById,
          bestIndexTimeOfDay,
          pastEventCount,
        ),
      ),
    };
  }

  private static parseXlsx(
    filename: string | undefined,
    sheetNames: string[],
    sheets: Record<string, Sheet>,
  ): string[][] {
    const lastRealSheetName = sheetNames
      .filter((name) => name.trim().toLowerCase() !== 'calculations')
      .reverse()[0];
    const sheet = sheets[lastRealSheetName];
    const csvString = xlsxUtils.sheet_to_csv(sheet, {
      strip: true,
      skipHidden: true,
    });

    const rows = parse(csvString, {
      columns: false,
      ltrim: true,
      rtrim: true,
      relaxColumnCount: true,
    });

    if (rows.length >= 5) {
      return rows;
    } else if (sheetNames.length > 1) {
      return this.parseXlsx(
        filename,
        sheetNames.filter((n) => n !== lastRealSheetName),
        sheets,
      );
    } else {
      throw new Error(`File ${filename} contains no non-empty sheets`);
    }
  }

  private buildDriver<T extends Omit<ChampionshipDriver, 'totalPoints'>>(
    driverId: string,
    previousDrivers: Record<string, T>,
    driversForEventById: Record<string, Driver>,
    bestPaxTimeOfDay: number,
    pastEventCount: number,
  ): ChampionshipDriver {
    const driverHistory = previousDrivers[driverId];
    const driverNewResults = driversForEventById[driverId];
    if (driverHistory && driverNewResults) {
      const newPoints = [
        ...driverHistory.points,
        calculatePointsForDriver(
          bestPaxTimeOfDay,
          driverNewResults,
          this.paxService.getMultiplierFromLongName(driverNewResults.carClass),
        ),
      ];
      return {
        ...driverHistory,
        points: newPoints,
        totalPoints: ChampionshipResultsParser.sumPoints(newPoints),
      };
    } else if (driverHistory) {
      const newPoints = [...driverHistory.points, 0];
      return {
        ...driverHistory,
        points: newPoints,
        totalPoints: ChampionshipResultsParser.sumPoints(newPoints),
      };
    } else {
      const newDriver = driversForEventById[driverId];
      const newPoints = [
        ...new Array(pastEventCount).fill(0),
        calculatePointsForDriver(
          bestPaxTimeOfDay,
          newDriver,
          this.paxService.getMultiplierFromLongName(newDriver.carClass),
        ),
      ];
      return {
        id: driverId,
        name: newDriver.name,
        points: newPoints,
        totalPoints: ChampionshipResultsParser.sumPoints(newPoints),
      };
    }
  }

  private static sumPoints(points: (number | undefined)[]): number {
    const eventCount = points.length;
    const fleshedOutPoints = points.map((p) => p || 0);
    if (eventCount < 4) {
      return fleshedOutPoints.reduce((sum, p) => sum + p, 0);
    } else {
      const eventsToCount = this.calculateEventsToCount(points.length);
      return fleshedOutPoints
        .sort()
        .reverse()
        .slice(0, eventsToCount)
        .reduce((sum, p) => sum + p, 0);
    }
  }

  static calculateEventsToCount(totalEventCount: number): number {
    if (totalEventCount < 4) return totalEventCount;
    else return Math.round(totalEventCount / 2) + 2;
  }
}
