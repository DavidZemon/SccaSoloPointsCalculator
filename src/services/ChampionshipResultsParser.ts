import { read as xlsxRead, Sheet, utils as xlsxUtils } from 'xlsx';
import { toast } from 'react-toastify';
import parse from 'csv-parse/lib/sync';
import {
  ChampionshipDriver,
  ChampionshipResults,
  ChampionshipType,
  Driver,
  EventResults,
  IndexedChampionshipResults,
  IndexedChampionshipType,
} from '../models';
import { PaxService } from './PaxService';

export class ChampionshipResultsParser {
  constructor(private readonly paxService: PaxService) {}

  async parse(
    inputFiles: Record<ChampionshipType, File | undefined>,
    eventResults: EventResults,
  ): Promise<ChampionshipResults> {
    const allDriversForEvent = Object.values(eventResults)
      .map((classCategory) => Object.values(classCategory))
      .flat()
      .map((classResults) => classResults.drivers)
      .flat();
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
                break;
              case 'PAX':
                const bestPaxTimeOfDay = Math.min(
                  ...allDriversForEvent.map(
                    (driver) =>
                      driver.bestLap().time! *
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
                      driver.bestLap().time! *
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
            }
          } catch (e) {
            toast.error(e.message ? e.message : e.toString());
          }
        }),
    );
    return results;
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

    const totalEvents = Object.values(previousDrivers)[0].points.length + 1;
    return {
      organization: rows[0][0],
      year: parseInt(rows[1][0].split(' ')[0]),
      drivers: [
        ...new Set([
          ...Object.keys(driversForEventById),
          ...Object.keys(previousDrivers),
        ]),
      ].map((driverId): ChampionshipDriver => {
        const driverHistory = previousDrivers[driverId];
        const driverNewResults = driversForEventById[driverId];
        if (driverHistory && driverNewResults) {
          const newPoints = [
            ...driverHistory.points,
            this.calculatePointsForDriver(bestIndexTimeOfDay, driverNewResults),
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
            ...new Array(totalEvents - 1).fill(0),
            this.calculatePointsForDriver(bestIndexTimeOfDay, newDriver),
          ];
          return {
            id: driverId,
            name: newDriver.name,
            points: newPoints,
            totalPoints: ChampionshipResultsParser.sumPoints(newPoints),
          };
        }
      }),
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

  private calculatePointsForDriver(fastest: number, driver: Driver): number {
    const actual =
      (driver.bestLap().time || Infinity) *
      this.paxService.getMultiplierFromLongName(driver.carClass);
    if (fastest === actual) {
      return 10000;
    } else {
      return Math.round((fastest / actual) * 10_000);
    }
  }

  private static sumPoints(points: (number | undefined)[]): number {
    const eventCount = points.length;
    const fleshedOutPoints = points.map((p) => p || 0);
    if (eventCount < 4) {
      return fleshedOutPoints.reduce((sum, p) => sum + p, 0);
    } else {
      const eventsToCount = this.calculateEventsToCount(points);
      return fleshedOutPoints
        .sort()
        .reverse()
        .slice(0, eventsToCount)
        .reduce((sum, p) => sum + p, 0);
    }
  }

  static calculateEventsToCount(points: (number | undefined)[]): number {
    if (points.length < 4) return points.length;
    else return Math.round(points.length / 2) + 2;
  }
}
