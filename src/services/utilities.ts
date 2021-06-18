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
