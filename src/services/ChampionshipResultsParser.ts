import { ChampionshipResults, ChampionshipType } from '../models';

export class ChampionshipResultsParser {
  async parse(
    results: Record<ChampionshipType, File | undefined>,
  ): Promise<ChampionshipResults> {
    return {};
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
