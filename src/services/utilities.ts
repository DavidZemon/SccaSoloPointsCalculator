import { Driver } from '../models';

export const SPECIAL_CLASS_MAP: Record<string, string> = {
  'Fun Class': 'FUN',
};

export function toShortClassName(longClassName: string): string {
  return (
    SPECIAL_CLASS_MAP[longClassName] ||
    longClassName
      .split(' ')
      .map((word) => word[0])
      .join('')
  );
}

export function calculatePointsForDriver(
  fastest: number,
  driver: Driver,
  paxMultiplier = 1,
): number {
  const actual = (driver.bestLap().time || Infinity) * paxMultiplier;
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
