import { read as xlsxRead, Sheet, utils as xlsxUtils } from 'xlsx';
import { toast } from 'react-toastify';
import parse from 'csv-parse/lib/sync';
import {
  ChampionshipResults,
  ChampionshipType,
  IndexedChampionshipResults,
  IndexedChampionshipType,
} from '../models';

export class ChampionshipResultsParser {
  async parse(
    inputFiles: Record<ChampionshipType, File | undefined>,
  ): Promise<ChampionshipResults> {
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
              default:
                results[championshipType as IndexedChampionshipType] =
                  this.parseIndexedResults(rows);
            }
          } catch (e) {
            toast.error(e.message ? e.message : e.toString());
          }
        }),
    );
    console.log(JSON.stringify(results, null, 2));
    return results;
  }

  private parseIndexedResults(rows: string[][]): IndexedChampionshipResults {
    return {
      organization: rows[0][0],
      year: parseInt(rows[1][0].split(' ')[0]),
      results: rows.slice(3).map((row) => ({
        position: parseInt(row[0]),
        name: row[1],
        points: row.slice(2, row.length - 2).map((p) => parseInt(p)),
      })),
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

  private static calculateClassPoints(fastest: number, actual: number): number {
    if (fastest === actual) {
      return 10000;
    } else {
      return Math.round((fastest / actual) * 10_000);
    }
  }

  private static calculateChampionshipPoints(
    points: (number | undefined)[],
  ): number {
    const eventCount = points.length;
    const fleshedOutPoints = points.map((p) => p || 0);
    if (eventCount < 4) {
      return fleshedOutPoints.reduce((sum, p) => sum + p, 0);
    } else {
      const eventsToCount = Math.round(eventCount / 2) + 2;
      return fleshedOutPoints
        .sort()
        .reverse()
        .slice(0, eventsToCount)
        .reduce((sum, p) => sum + p, 0);
    }
  }
}
