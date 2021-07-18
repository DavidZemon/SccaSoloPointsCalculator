export type TimeSelection = 'day1' | 'day2' | 'combined';
export type IndexedChampionshipType = 'PAX' | 'Novice' | 'Ladies';
export type ChampionshipType = 'Class' | IndexedChampionshipType;

export class LapTime {
  public static readonly DSQ = new LapTime(0, 0, 'DSQ');
  public static readonly DNS = new LapTime(0, 0, 'DNS');

  public readonly raw?: number;
  public readonly time?: number;
  public readonly cones: number;
  public readonly dnf: boolean;
  public readonly rerun: boolean;
  public readonly dsq: boolean;
  public readonly dns: boolean;

  constructor(rawTime: number, cones: number, penalty?: string) {
    switch (penalty) {
      case 'DNF':
        this.dnf = true;
        this.rerun = false;
        this.dsq = false;
        this.dns = false;
        this.cones = 0;
        break;
      case 'RRN':
        this.dnf = false;
        this.rerun = true;
        this.dsq = false;
        this.dns = false;
        this.cones = 0;
        break;
      case 'DSQ':
        this.dnf = false;
        this.rerun = false;
        this.dsq = true;
        this.dns = false;
        this.cones = 0;
        break;
      case 'DNS':
        this.dnf = false;
        this.rerun = false;
        this.dsq = false;
        this.dns = true;
        this.cones = 0;
        break;
      default:
        this.dnf = false;
        this.rerun = false;
        this.dsq = false;
        this.dns = false;
        this.raw = rawTime;
        this.time = rawTime + cones * 2;
        this.cones = cones;
    }
  }

  toString(paxMultiplier?: number, displayConeCount = true): string {
    if (this.dnf) return 'DNF';
    else if (this.rerun) return 'Re-run';
    else if (this.dsq) return 'DSQ';
    else if (this.dns) return 'DNS';
    else {
      let time = this.time!;
      if (paxMultiplier) time *= paxMultiplier;
      if (displayConeCount)
        return `${time.toFixed(3)}` + (this.cones ? ` (${this.cones})` : '');
      else return time.toFixed(3);
    }
  }

  add(rhs: LapTime): LapTime {
    if (this.dnf || this.rerun || this.dsq || this.dns) {
      return this;
    } else if (rhs.dnf || rhs.rerun || rhs.dsq || rhs.dns) {
      return rhs;
    } else {
      return new LapTime(this.raw! + rhs.raw!, this.cones + rhs.cones);
    }
  }

  static compare(lhs: LapTime, rhs: LapTime): number {
    if (lhs.time === rhs.time) {
      return 0;
    } else if (lhs.time === undefined) {
      return 1;
    } else if (rhs.time === undefined) {
      return -1;
    } else {
      return lhs.time - rhs.time;
    }
  }
}

export interface ExportedDriver {
  Position?: number;
  Class: string;
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
  day1?: LapTime[];
  day2?: LapTime[];
}

export class Driver {
  readonly id: string;
  readonly name: string;
  readonly carNumber: number;
  readonly carClass: string;
  readonly carDescription: string;
  readonly region: string;
  readonly rookie: boolean;
  readonly ladiesChampionship: boolean;
  position?: number;
  readonly dsq: boolean;
  readonly paxMultiplier: number;

  private readonly day1Times?: LapTime[];
  private readonly day2Times?: LapTime[];
  private readonly combined: LapTime;

  constructor(driver: ExportedDriver) {
    this.rookie = !!driver.Rookie;
    this.ladiesChampionship = !!driver.Ladies;
    this.carNumber = driver.Number;
    this.carClass = driver.Class;
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
        : LapTime.DNS;
  }

  bestLap(timeSelection: TimeSelection = 'day1'): LapTime {
    if (this.dsq) return LapTime.DSQ;
    else {
      switch (timeSelection) {
        case 'day1':
          return [...(this.getTimes('day1') || [LapTime.DNS])].sort(
            LapTime.compare,
          )[0];
        case 'day2':
          return [...(this.getTimes('day2') || [LapTime.DNS])].sort(
            LapTime.compare,
          )[0];
        case 'combined':
          return this.combined;
      }
    }
  }

  getTimes(
    timeSelect: Exclude<TimeSelection, 'combined'> = 'day1',
  ): LapTime[] | undefined {
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
  public readonly carClass: string;
  public readonly drivers: Driver[];

  constructor(carClass: string) {
    this.trophyCount = 0;
    this.carClass = carClass;
    this.drivers = [];
  }

  getBestInClass(timeSelection: TimeSelection = 'day1'): number | undefined {
    return this.drivers[0].bestLap(timeSelection).time;
  }
}

export type EventResults = Record<string, ClassResults>;

export interface ChampionshipDriver {
  id: string;
  name: string;
  points: number[];
  totalPoints: number;
}

export interface ClassChampionshipDriver extends ChampionshipDriver {
  carClass: string;
}

export interface IndexedChampionshipResults {
  year: number;
  organization: string;
  drivers: ChampionshipDriver[];
}

export interface ClassChampionshipResults {
  year: number;
  organization: string;
  driversByClass: Record<string, ChampionshipDriver[]>;
}

export interface ChampionshipResults {
  Class?: ClassChampionshipResults;
  PAX?: IndexedChampionshipResults;
  Novice?: IndexedChampionshipResults;
  Ladies?: IndexedChampionshipResults;
}
