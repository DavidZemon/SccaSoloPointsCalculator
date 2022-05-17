import { Driver } from './rust_helpers';
import { ChampionshipDriver, ClassChampionshipDriver } from '../models';

export function calculatePointsForDriver(
  fastest: number,
  driver: Driver,
  paxMultiplier = 1,
): number {
  const actual = (driver.best_lap().time || Infinity) * paxMultiplier;
  if (fastest === actual) {
    return 10000;
  } else {
    return Math.round((fastest / actual) * 10_000);
  }
}

/**
 * Calculate number of trophies to be awarded for the given number of drivers
 * @param drivers Either an array of drivers, or a count for the number of drivers in the group
 * @returns Number of trophies for the group of drivers
 */
export function calculateTrophies(drivers: any[] | number): number {
  const driverCount = Array.isArray(drivers) ? drivers.length : drivers;
  if (driverCount <= 1) return 0;
  else if (driverCount >= 10) {
    return 3 + Math.ceil((driverCount - 9) / 4);
  } else {
    return Math.ceil(driverCount / 3);
  }
}

export function calculateClassChampionshipTrophies(
  drivers: ClassChampionshipDriver[],
): number {
  if (drivers.length <= 1) return 0;
  else {
    const totalEventCount = drivers[0].points.length;
    const eventCountCutoff = Math.ceil(totalEventCount / 2) + 2;
    const averageDriverCount =
      drivers
        .map((driver) => driver.points)
        .flat()
        .filter((points) => points).length / totalEventCount;

    const defaultTrophyCount = calculateTrophies(
      Math.round(averageDriverCount),
    );
    const overrideTrophyCount = drivers.filter(
      (driver) =>
        // Average points higher than 9600
        driver.totalPoints >= eventCountCutoff * 9600 &&
        // Driver attended enough events to qualify
        driver.points.filter((points) => points).length >= eventCountCutoff,
    ).length;
    return Math.max(defaultTrophyCount, overrideTrophyCount);
  }
}

export function doesIndexDriverGetATrophy(
  driver: ChampionshipDriver,
  position: number,
): boolean {
  if (position >= 3) return false;
  else {
    const totalEventCount = driver.points.length;
    const eventCountCutoff = Math.ceil(totalEventCount / 2) + 2;
    const attendanceCount = driver.points.filter((points) => points).length;
    return attendanceCount >= eventCountCutoff;
  }
}
