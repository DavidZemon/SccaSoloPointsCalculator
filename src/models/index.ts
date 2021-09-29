export type TimeSelection = 'day1' | 'day2' | 'combined';
export type IndexedChampionshipType = 'PAX' | 'Novice' | 'Ladies';
export type ChampionshipType = 'Class' | IndexedChampionshipType;

export type ClassCategory =
  | 'Street Category'
  | 'Street Touring Category'
  | 'Street Prepared Category'
  | 'Street Modified Category'
  | 'Prepared Category'
  | 'Modified Category'
  | 'Classic American Muscle Category'
  | 'Xtreme Street'
  | 'Miscellaneous Category';

export type ShortCarClass =
  | 'SS'
  | 'AS'
  | 'BS'
  | 'CS'
  | 'DS'
  | 'ES'
  | 'FS'
  | 'GS'
  | 'HS'
  | 'STH'
  | 'STU'
  | 'STX'
  | 'STR'
  | 'STS'
  | 'SSP'
  | 'ASP'
  | 'BSP'
  | 'CSP'
  | 'DSP'
  | 'ESP'
  | 'FSP'
  | 'SSM'
  | 'SM'
  | 'SMF'
  | 'XP'
  | 'CP'
  | 'DP'
  | 'EP'
  | 'FP'
  | 'AM'
  | 'BM'
  | 'CM'
  | 'DM'
  | 'EM'
  | 'FM'
  | 'KM'
  | 'CAMC'
  | 'CAMT'
  | 'CAMS'
  | 'XSA'
  | 'XSB'
  | 'EVX'
  | 'SSC'
  | 'SSL'
  | 'ASL'
  | 'BSL'
  | 'CSL'
  | 'DSL'
  | 'ESL'
  | 'FSL'
  | 'GSL'
  | 'HSL'
  | 'STHL'
  | 'STUL'
  | 'STXL'
  | 'STRL'
  | 'STSL'
  | 'SSPL'
  | 'ASPL'
  | 'BSPL'
  | 'CSPL'
  | 'DSPL'
  | 'ESPL'
  | 'FSPL'
  | 'SSML'
  | 'SML'
  | 'SMFL'
  | 'XPL'
  | 'CPL'
  | 'DPL'
  | 'EPL'
  | 'FPL'
  | 'AML'
  | 'BML'
  | 'CML'
  | 'DML'
  | 'EML'
  | 'FML'
  | 'KML'
  | 'CAMCL'
  | 'CAMTL'
  | 'CAMSL'
  | 'XSAL'
  | 'XSBL'
  | 'EVXL'
  | 'SSCL'
  | 'FUN';

export type LongCarClass =
  | 'Super Street'
  | 'A Street'
  | 'B Street'
  | 'C Street'
  | 'D Street'
  | 'E Street'
  | 'F Street'
  | 'G Street'
  | 'H Street'
  | 'Street Touring Hatchback'
  | 'Street Touring Ultra'
  | 'Street Touring Xtreme'
  | 'Street Touring Roadster'
  | 'Street Touring Sport'
  | 'Super Street Prepared'
  | 'A Street Prepared'
  | 'B Street Prepared'
  | 'C Street Prepared'
  | 'D Street Prepared'
  | 'E Street Prepared'
  | 'F Street Prepared'
  | 'Super Street Modified'
  | 'Street Modified'
  | 'Street Modified Front-Wheel-Drive'
  | 'X Prepared'
  | 'C Prepared'
  | 'D Prepared'
  | 'E Prepared'
  | 'F Prepared'
  | 'A Modified'
  | 'B Modified'
  | 'C Modified'
  | 'D Modified'
  | 'E Modified'
  | 'F Modified'
  | 'Kart Modified'
  | 'Classic American Muscle Contemporary'
  | 'Classic American Muscle Traditional'
  | 'Classic American Muscle Sports'
  | 'Xtreme Street A'
  | 'Xtreme Street B'
  | 'Electric Vehicle Xtreme'
  | 'Solo Spec Coupe'
  | 'Super Street Ladies'
  | 'A Street Ladies'
  | 'B Street Ladies'
  | 'C Street Ladies'
  | 'D Street Ladies'
  | 'E Street Ladies'
  | 'F Street Ladies'
  | 'G Street Ladies'
  | 'H Street Ladies'
  | 'Street Touring Hatchback Ladies'
  | 'Street Touring Ultra Ladies'
  | 'Street Touring Xtreme Ladies'
  | 'Street Touring Roadster Ladies'
  | 'Street Touring Sport Ladies'
  | 'Super Street Prepared Ladies'
  | 'A Street Prepared Ladies'
  | 'B Street Prepared Ladies'
  | 'C Street Prepared Ladies'
  | 'D Street Prepared Ladies'
  | 'E Street Prepared Ladies'
  | 'F Street Prepared Ladies'
  | 'Super Street Modified Ladies'
  | 'Street Modified Ladies'
  | 'Street Modified Front-Wheel-Drive Ladies'
  | 'X Prepared Ladies'
  | 'C Prepared Ladies'
  | 'D Prepared Ladies'
  | 'E Prepared Ladies'
  | 'F Prepared Ladies'
  | 'A Modified Ladies'
  | 'B Modified Ladies'
  | 'C Modified Ladies'
  | 'D Modified Ladies'
  | 'E Modified Ladies'
  | 'F Modified Ladies'
  | 'Kart Modified Ladies'
  | 'Classic American Muscle Contemporary Ladies'
  | 'Classic American Muscle Traditional Ladies'
  | 'Classic American Muscle Sports Ladies'
  | 'Xtreme Street A Ladies'
  | 'Xtreme Street B Ladies'
  | 'Electric Vehicle Xtreme Ladies'
  | 'Solo Spec Coupe Ladies'
  | 'Fun';

export const CLASS_MAP: Record<ShortCarClass, CarClass> = {
  SS: { short: 'SS', long: 'Super Street', category: 'Street Category' },
  AS: { short: 'AS', long: 'A Street', category: 'Street Category' },
  BS: { short: 'BS', long: 'B Street', category: 'Street Category' },
  CS: { short: 'CS', long: 'C Street', category: 'Street Category' },
  DS: { short: 'DS', long: 'D Street', category: 'Street Category' },
  ES: { short: 'ES', long: 'E Street', category: 'Street Category' },
  FS: { short: 'FS', long: 'F Street', category: 'Street Category' },
  GS: { short: 'GS', long: 'G Street', category: 'Street Category' },
  HS: { short: 'HS', long: 'H Street', category: 'Street Category' },
  STH: {
    short: 'STH',
    long: 'Street Touring Hatchback',
    category: 'Street Touring Category',
  },
  STU: {
    short: 'STU',
    long: 'Street Touring Ultra',
    category: 'Street Touring Category',
  },
  STX: {
    short: 'STX',
    long: 'Street Touring Xtreme',
    category: 'Street Touring Category',
  },
  STR: {
    short: 'STR',
    long: 'Street Touring Roadster',
    category: 'Street Touring Category',
  },
  STS: {
    short: 'STS',
    long: 'Street Touring Sport',
    category: 'Street Touring Category',
  },
  SSP: {
    short: 'SSP',
    long: 'Super Street Prepared',
    category: 'Street Touring Category',
  },
  ASP: {
    short: 'ASP',
    long: 'A Street Prepared',
    category: 'Street Prepared Category',
  },
  BSP: {
    short: 'BSP',
    long: 'B Street Prepared',
    category: 'Street Prepared Category',
  },
  CSP: {
    short: 'CSP',
    long: 'C Street Prepared',
    category: 'Street Prepared Category',
  },
  DSP: {
    short: 'DSP',
    long: 'D Street Prepared',
    category: 'Street Prepared Category',
  },
  ESP: {
    short: 'ESP',
    long: 'E Street Prepared',
    category: 'Street Prepared Category',
  },
  FSP: {
    short: 'FSP',
    long: 'F Street Prepared',
    category: 'Street Prepared Category',
  },
  SSM: {
    short: 'SSM',
    long: 'Super Street Modified',
    category: 'Street Modified Category',
  },
  SM: {
    short: 'SM',
    long: 'Street Modified',
    category: 'Street Modified Category',
  },
  SMF: {
    short: 'SMF',
    long: 'Street Modified Front-Wheel-Drive',
    category: 'Street Modified Category',
  },
  XP: { short: 'XP', long: 'X Prepared', category: 'Modified Category' },
  CP: { short: 'CP', long: 'C Prepared', category: 'Modified Category' },
  DP: { short: 'DP', long: 'D Prepared', category: 'Modified Category' },
  EP: { short: 'EP', long: 'E Prepared', category: 'Modified Category' },
  FP: { short: 'FP', long: 'F Prepared', category: 'Modified Category' },
  AM: { short: 'AM', long: 'A Modified', category: 'Modified Category' },
  BM: { short: 'BM', long: 'B Modified', category: 'Modified Category' },
  CM: { short: 'CM', long: 'C Modified', category: 'Modified Category' },
  DM: { short: 'DM', long: 'D Modified', category: 'Modified Category' },
  EM: { short: 'EM', long: 'E Modified', category: 'Modified Category' },
  FM: { short: 'FM', long: 'F Modified', category: 'Modified Category' },
  KM: { short: 'KM', long: 'Kart Modified', category: 'Modified Category' },
  CAMC: {
    short: 'CAMC',
    long: 'Classic American Muscle Contemporary',
    category: 'Classic American Muscle Category',
  },
  CAMT: {
    short: 'CAMT',
    long: 'Classic American Muscle Traditional',
    category: 'Classic American Muscle Category',
  },
  CAMS: {
    short: 'CAMS',
    long: 'Classic American Muscle Sports',
    category: 'Classic American Muscle Category',
  },
  XSA: { short: 'XSA', long: 'Xtreme Street A', category: 'Xtreme Street' },
  XSB: { short: 'XSB', long: 'Xtreme Street B', category: 'Xtreme Street' },
  EVX: {
    short: 'EVX',
    long: 'Electric Vehicle Xtreme',
    category: 'Miscellaneous Category',
  },
  SSC: {
    short: 'SSC',
    long: 'Solo Spec Coupe',
    category: 'Miscellaneous Category',
  },
  SSL: {
    short: 'SSL',
    long: 'Super Street Ladies',
    category: 'Street Category',
  },
  ASL: { short: 'ASL', long: 'A Street Ladies', category: 'Street Category' },
  BSL: { short: 'BSL', long: 'B Street Ladies', category: 'Street Category' },
  CSL: { short: 'CSL', long: 'C Street Ladies', category: 'Street Category' },
  DSL: { short: 'DSL', long: 'D Street Ladies', category: 'Street Category' },
  ESL: { short: 'ESL', long: 'E Street Ladies', category: 'Street Category' },
  FSL: { short: 'FSL', long: 'F Street Ladies', category: 'Street Category' },
  GSL: { short: 'GSL', long: 'G Street Ladies', category: 'Street Category' },
  HSL: { short: 'HSL', long: 'H Street Ladies', category: 'Street Category' },
  STHL: {
    short: 'STHL',
    long: 'Street Touring Hatchback Ladies',
    category: 'Street Touring Category',
  },
  STUL: {
    short: 'STUL',
    long: 'Street Touring Ultra Ladies',
    category: 'Street Touring Category',
  },
  STXL: {
    short: 'STXL',
    long: 'Street Touring Xtreme Ladies',
    category: 'Street Touring Category',
  },
  STRL: {
    short: 'STRL',
    long: 'Street Touring Roadster Ladies',
    category: 'Street Touring Category',
  },
  STSL: {
    short: 'STSL',
    long: 'Street Touring Sport Ladies',
    category: 'Street Touring Category',
  },
  SSPL: {
    short: 'SSPL',
    long: 'Super Street Prepared Ladies',
    category: 'Street Prepared Category',
  },
  ASPL: {
    short: 'ASPL',
    long: 'A Street Prepared Ladies',
    category: 'Street Prepared Category',
  },
  BSPL: {
    short: 'BSPL',
    long: 'B Street Prepared Ladies',
    category: 'Street Prepared Category',
  },
  CSPL: {
    short: 'CSPL',
    long: 'C Street Prepared Ladies',
    category: 'Street Prepared Category',
  },
  DSPL: {
    short: 'DSPL',
    long: 'D Street Prepared Ladies',
    category: 'Street Prepared Category',
  },
  ESPL: {
    short: 'ESPL',
    long: 'E Street Prepared Ladies',
    category: 'Street Prepared Category',
  },
  FSPL: {
    short: 'FSPL',
    long: 'F Street Prepared Ladies',
    category: 'Street Prepared Category',
  },
  SSML: {
    short: 'SSML',
    long: 'Super Street Modified Ladies',
    category: 'Street Modified Category',
  },
  SML: {
    short: 'SML',
    long: 'Street Modified Ladies',
    category: 'Street Modified Category',
  },
  SMFL: {
    short: 'SMFL',
    long: 'Street Modified Front-Wheel-Drive Ladies',
    category: 'Street Modified Category',
  },
  XPL: {
    short: 'XPL',
    long: 'X Prepared Ladies',
    category: 'Prepared Category',
  },
  CPL: {
    short: 'CPL',
    long: 'C Prepared Ladies',
    category: 'Prepared Category',
  },
  DPL: {
    short: 'DPL',
    long: 'D Prepared Ladies',
    category: 'Prepared Category',
  },
  EPL: {
    short: 'EPL',
    long: 'E Prepared Ladies',
    category: 'Prepared Category',
  },
  FPL: {
    short: 'FPL',
    long: 'F Prepared Ladies',
    category: 'Prepared Category',
  },
  AML: {
    short: 'AML',
    long: 'A Modified Ladies',
    category: 'Modified Category',
  },
  BML: {
    short: 'BML',
    long: 'B Modified Ladies',
    category: 'Modified Category',
  },
  CML: {
    short: 'CML',
    long: 'C Modified Ladies',
    category: 'Modified Category',
  },
  DML: {
    short: 'DML',
    long: 'D Modified Ladies',
    category: 'Modified Category',
  },
  EML: {
    short: 'EML',
    long: 'E Modified Ladies',
    category: 'Modified Category',
  },
  FML: {
    short: 'FML',
    long: 'F Modified Ladies',
    category: 'Modified Category',
  },
  KML: {
    short: 'KML',
    long: 'Kart Modified Ladies',
    category: 'Modified Category',
  },
  CAMCL: {
    short: 'CAMCL',
    long: 'Classic American Muscle Contemporary Ladies',
    category: 'Classic American Muscle Category',
  },
  CAMTL: {
    short: 'CAMTL',
    long: 'Classic American Muscle Traditional Ladies',
    category: 'Classic American Muscle Category',
  },
  CAMSL: {
    short: 'CAMSL',
    long: 'Classic American Muscle Sports Ladies',
    category: 'Classic American Muscle Category',
  },
  XSAL: {
    short: 'XSAL',
    long: 'Xtreme Street A Ladies',
    category: 'Xtreme Street',
  },
  XSBL: {
    short: 'XSBL',
    long: 'Xtreme Street B Ladies',
    category: 'Xtreme Street',
  },
  EVXL: {
    short: 'EVXL',
    long: 'Electric Vehicle Xtreme Ladies',
    category: 'Miscellaneous Category',
  },
  SSCL: {
    short: 'SSCL',
    long: 'Solo Spec Coupe Ladies',
    category: 'Miscellaneous Category',
  },
  FUN: {
    short: 'FUN',
    long: 'Fun',
    category: 'Miscellaneous Category',
  },
};

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
  day1?: LapTime[];
  day2?: LapTime[];
}

export interface CarClass {
  short: ShortCarClass;
  long: LongCarClass;
  category: ClassCategory;
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
  readonly carClass: CarClass;
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
    this.error =
      !driver['Runs Day1'] && !driver['Runs Day2'] && !!driver['Best Run'];
    this.rookie = !!driver.Rookie;
    this.ladiesChampionship = !!driver.Ladies;
    this.carNumber = driver.Number;
    this.carClass = CLASS_MAP[driver.Class];
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
  public readonly carClass: CarClass;
  public readonly drivers: Driver[];

  constructor(carClass: ShortCarClass) {
    this.trophyCount = 0;
    this.carClass = CLASS_MAP[carClass];
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
  carClass: CarClass;
}

export interface IndexedChampionshipResults {
  year: number;
  organization: string;
  drivers: ChampionshipDriver[];
}

export interface ClassChampionshipResults {
  year: number;
  organization: string;
  driversByClass: Record<ShortCarClass, ChampionshipDriver[]>;
}

export interface ChampionshipResults {
  Class?: ClassChampionshipResults;
  PAX?: IndexedChampionshipResults;
  Novice?: IndexedChampionshipResults;
  Ladies?: IndexedChampionshipResults;
}
