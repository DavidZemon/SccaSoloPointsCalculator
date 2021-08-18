import { read as xlsxRead, Sheet, utils as xlsxUtils } from 'xlsx';
import { toast } from 'react-toastify';
import parse from 'csv-parse/lib/sync';
import {
  ChampionshipDriver,
  ChampionshipResults,
  ChampionshipType,
  CLASS_MAP,
  ClassChampionshipDriver,
  ClassChampionshipResults,
  Driver,
  DriverId,
  EventResults,
  IndexedChampionshipResults,
  IndexedChampionshipType,
  ShortCarClass,
} from '../models';
import { calculatePointsForDriver } from './utilities';

export class ChampionshipResultsParser {
  async parse(
    inputFiles: Partial<Record<ChampionshipType, File>>,
    eventResults: EventResults,
    newLadies: string[],
  ): Promise<ChampionshipResults> {
    const allDriversForEvent = Object.values(eventResults)
      .filter((classResults) => classResults.carClass !== CLASS_MAP.FUN)
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
                      (driver.bestLap().time || Infinity) *
                      driver.paxMultiplier,
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
                      (driver.bestLap().time || Infinity) *
                      driver.paxMultiplier,
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
                      (driver.bestLap().time || Infinity) *
                      driver.paxMultiplier,
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
    const rowsByClassAndDriverId: Partial<
      Record<
        ShortCarClass,
        Record<DriverId, Omit<ClassChampionshipDriver, 'totalPoints'>>
      >
    > = {};
    let rowsForOneClass: Record<
      DriverId,
      Omit<ClassChampionshipDriver, 'totalPoints'>
    >;
    let currentClass: ShortCarClass;

    rows
      .slice(ChampionshipResultsParser.getHeaderRowIndex(rows) + 1)
      .forEach((row) => {
        // If the first cell is non-numeric, it is a class header
        if (isNaN(parseInt(row[0]))) {
          const delimiter = row[0].includes(' - ') ? ' - ' : ' – ';
          currentClass = row[0].split(delimiter)[0] as ShortCarClass;
          if (!currentClass) {
            currentClass = row[0].split(delimiter)[0] as ShortCarClass;
          }
          rowsByClassAndDriverId[currentClass] = rowsForOneClass = {};
        } else {
          const id = row[1].toLowerCase().trim();
          rowsForOneClass[id] = {
            carClass: CLASS_MAP[currentClass],
            id,
            name: row[1],
            points: row.slice(2, row.length - 2).map((p) => parseInt(p)),
          };
        }
      });

    const newEventDriversByClassAndId: Partial<
      Record<ShortCarClass, Record<DriverId, Driver>>
    > = {};

    // Get this event's driver IDs
    Object.values(eventResults)
      .filter((classResults) => classResults.carClass !== CLASS_MAP.FUN)
      .forEach((classResults) => {
        newEventDriversByClassAndId[classResults.carClass.short] =
          classResults.drivers
            .filter((driver) => !driver.dsq)
            .reduce((o, d) => {
              o[d.id] = d;
              return o;
            }, {} as Record<string, Driver>);
      });

    const allDriverIdsByClass: Partial<Record<ShortCarClass, DriverId[]>> = {};
    (
      [
        ...new Set([
          ...Object.keys(rowsByClassAndDriverId),
          ...Object.keys(newEventDriversByClassAndId),
        ]),
      ] as ShortCarClass[]
    ).forEach((carClass) => {
      allDriverIdsByClass[carClass] = [
        ...new Set([
          ...Object.keys(rowsByClassAndDriverId[carClass] || []),
          ...Object.keys(newEventDriversByClassAndId[carClass] || []),
        ]),
      ];
    });

    const driversByClass: Record<string, ChampionshipDriver[]> = {};
    (
      Object.entries(allDriverIdsByClass) as [ShortCarClass, DriverId[]][]
    ).forEach(([carClass, driverIds]) => {
      const classHistory = rowsByClassAndDriverId[carClass] || {};
      const newEventDriversById = newEventDriversByClassAndId[carClass] || {};

      const bestTimeOfDay = Math.min(
        ...Object.values(newEventDriversById)
          .map(
            (driver) =>
              [driver.bestLap().time, driver.paxMultiplier] as [
                number | undefined,
                number,
              ],
          )
          .filter(([t, _]) => t !== undefined)
          .map(([t, multiplier]) => t! * multiplier),
      );
      driversByClass[carClass] = driverIds.map(
        (driverId): ClassChampionshipDriver => ({
          ...ChampionshipResultsParser.buildDriver(
            driverId,
            classHistory,
            newEventDriversById,
            bestTimeOfDay,
            pastEventCount,
          ),
          carClass: CLASS_MAP[carClass],
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
    driversForEventById: Record<DriverId, Driver>,
    bestIndexTimeOfDay: number,
  ): IndexedChampionshipResults {
    const previousDrivers = rows
      .slice(ChampionshipResultsParser.getHeaderRowIndex(rows) + 1)
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
        ChampionshipResultsParser.buildDriver(
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
      blankrows: false,
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

  private static buildDriver<T extends Omit<ChampionshipDriver, 'totalPoints'>>(
    driverId: DriverId,
    previousDrivers: Record<DriverId, T>,
    driversForEventById: Record<DriverId, Driver>,
    bestPaxTimeOfDay: number,
    pastEventCount: number,
  ): ChampionshipDriver {
    const driverHistory = previousDrivers[driverId];
    const driverNewResults = driversForEventById[driverId];
    if (driverHistory && driverNewResults) {
      const pointsForNewEvent = calculatePointsForDriver(
        bestPaxTimeOfDay,
        driverNewResults,
        driverNewResults.paxMultiplier,
      );
      const newPointListing = [...driverHistory.points, pointsForNewEvent];
      return {
        ...driverHistory,
        points: newPointListing,
        totalPoints: ChampionshipResultsParser.sumPoints(newPointListing),
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
          newDriver.paxMultiplier,
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
        .sort((a, b) => b - a)
        .slice(0, eventsToCount)
        .reduce((sum, p) => sum + p, 0);
    }
  }

  private static getHeaderRowIndex(rows: string[][]): number {
    for (let i = 0; i < rows.length; ++i) {
      if (rows[i][0] === 'Rank' && rows[i][1] === 'Driver') return i;
    }
    throw new Error('Unable to find header row index');
  }

  static calculateEventsToCount(totalEventCount: number): number {
    if (totalEventCount < 4) return totalEventCount;
    else return Math.round(totalEventCount / 2) + 2;
  }
}
