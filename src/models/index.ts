import * as rusty from 'rusty/rusty';

export type TimeSelection = 'day1' | 'day2' | 'combined';
export type IndexedChampionshipType = 'PAX' | 'Novice' | 'Ladies';
export type ChampionshipType = 'Class' | IndexedChampionshipType;

export type ShortCarClass = keyof typeof rusty.ShortCarClass;

/**
 * Object with property names that match the Pronto export file
 */
export interface ExportedDriver {
  Position?: number;
  Class: ShortCarClass;
  Number: number;
  'First Name'?: string;
  'Last Name'?: string;
  'Car Year'?: number;
  'Car Make'?: string;
  'Car Model'?: string;
  'Car Color'?: string;
  'Member #'?: number;
  Rookie?: number;
  Ladies?: number;
  DSQ?: number;
  Region?: string;
  'Best Run': number | string;
  'Pax Index': number;
  'Pax Time': number;
  'Runs Day1'?: number;
  'Runs Day2'?: number;
  day1?: rusty.LapTime[];
  day2?: rusty.LapTime[];
}

export type DriverId = string;

/**
 * Object representing a driver which is easier to work with in a program.
 *
 * Same core data as `ExportedDriver`, but easier to work with.
 */
export class Driver {
  readonly error: boolean;
  readonly id: DriverId;
  readonly name: string;
  readonly carNumber: number;
  readonly carClass: rusty.CarClass;
  readonly carDescription: string;
  readonly region: string;
  readonly rookie: boolean;
  readonly ladiesChampionship: boolean;
  position?: number;
  readonly dsq: boolean;
  readonly paxMultiplier: number;

  private readonly day1Times?: rusty.LapTime[];
  private readonly day2Times?: rusty.LapTime[];
  private readonly combined: rusty.LapTime;

  constructor(driver: ExportedDriver) {
    this.error =
      !driver['Runs Day1'] && !driver['Runs Day2'] && !!driver['Best Run'];
    this.rookie = !!driver.Rookie;
    this.ladiesChampionship = !!driver.Ladies;
    this.carNumber = driver.Number;
    this.carClass = rusty.get_car_class(rusty.ShortCarClass[driver.Class])!;
    if (!this.carClass) {
      console.error(`Unable to map class "${driver.Class}"`);
    }
    this.name = `${driver['First Name']} ${driver['Last Name']}`;
    this.id = this.name.toLowerCase().trim();
    this.carDescription =
      `${driver['Car Year']} ${driver['Car Make']} ${driver['Car Model']}`.trim();
    this.region = driver.Region || '';
    this.dsq = false;
    this.paxMultiplier = driver['Pax Index'];
    this.day1Times = driver.day1;
    this.day2Times = driver.day2;
    this.combined =
      this.day1Times?.length && this.day2Times?.length
        ? this.bestLap('day1').add(this.bestLap('day2'))
        : rusty.LapTime.dns();
  }

  bestLap(timeSelection: TimeSelection = 'day1'): rusty.LapTime {
    if (this.dsq) return rusty.LapTime.dsq();
    else {
      switch (timeSelection) {
        case 'day1':
          return [...(this.getTimes('day1') || [rusty.LapTime.dns()])].sort(
            rusty.LapTime.compare,
          )[0];
        case 'day2':
          return [...(this.getTimes('day2') || [rusty.LapTime.dns()])].sort(
            rusty.LapTime.compare,
          )[0];
        case 'combined':
          return this.combined;
      }
    }
  }

  getTimes(
    timeSelect: Exclude<TimeSelection, 'combined'> = 'day1',
  ): rusty.LapTime[] | undefined {
    switch (timeSelect) {
      case 'day1':
        return this.day1Times;
      case 'day2':
        return this.day2Times;
    }
  }

  difference(
    fastestOfDay?: number,
    usePax = false,
    timeSelection: TimeSelection = 'day1',
  ): string {
    const timeToCompare = this.bestLap(timeSelection);
    const myTimeToCompare =
      (usePax ? this.paxMultiplier : 1) * (timeToCompare.time || Infinity);
    return timeToCompare.time
      ? myTimeToCompare === fastestOfDay
        ? ''
        : `${(fastestOfDay! - myTimeToCompare).toFixed(3)}`
      : 'N/A';
  }
}

export class ClassResults {
  public trophyCount: number;
  public readonly carClass: rusty.CarClass;
  public readonly drivers: Driver[];

  constructor(carClass: ShortCarClass) {
    this.trophyCount = 0;
    this.carClass = rusty.get_car_class(rusty.ShortCarClass[carClass])!;
    this.drivers = [];
  }

  getBestInClass(timeSelection: TimeSelection = 'day1'): number | undefined {
    return this.drivers[0].bestLap(timeSelection).time;
  }
}

export type EventResults = Partial<Record<ShortCarClass, ClassResults>>;

export interface ChampionshipDriver {
  /**
   * Unique(ish) ID for the driver
   */
  id: string;
  /**
   * Driver's full name
   */
  name: string;
  /**
   * Points that this driver has earned, sorted by event sequence
   */
  points: number[];
  /**
   * Total championship points value. Will not be equal to the sum of `ChampionshipDriver.points` if the driver has
   * participated in six or more events.
   */
  totalPoints: number;
}

export interface ClassChampionshipDriver extends ChampionshipDriver {
  /**
   * Class that this driver has participated in
   */
  carClass: rusty.CarClass;
}

export interface IndexedChampionshipResults {
  year: number;
  organization: string;
  drivers: ChampionshipDriver[];
}

export interface ClassChampionshipResults {
  year: number;
  organization: string;
  driversByClass: Record<ShortCarClass, ClassChampionshipDriver[]>;
}

export interface ChampionshipResults {
  Class?: ClassChampionshipResults;
  PAX?: IndexedChampionshipResults;
  Novice?: IndexedChampionshipResults;
  Ladies?: IndexedChampionshipResults;
}
