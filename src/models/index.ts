export class LapTime {
  public static readonly DSQ = new LapTime('DSQ');

  public readonly time?: number;
  public readonly cones: number;
  public readonly dnf: boolean;
  public readonly rerun: boolean;
  public readonly dsq: boolean;

  constructor(lap_time_str: string) {
    switch (lap_time_str) {
      case 'DNF':
        this.dnf = true;
        this.rerun = false;
        this.dsq = false;
        this.cones = 0;
        break;
      case 'RRN':
        this.dnf = false;
        this.rerun = true;
        this.dsq = false;
        this.cones = 0;
        break;
      case 'DSQ':
        this.dnf = false;
        this.rerun = false;
        this.dsq = true;
        this.cones = 0;
        break;
      default:
        this.dnf = false;
        this.rerun = false;
        this.dsq = false;
        const timeParts = lap_time_str.split('(');
        this.time = parseFloat(timeParts[0]);
        this.cones =
          timeParts.length === 2
            ? parseInt(timeParts[1].slice(0, timeParts[1].length - 1))
            : 0;
    }
  }

  toString(paxMultiplier?: number, displayConeCount = true): string {
    if (this.dnf) return 'DNF';
    else if (this.rerun) return 'Re-run';
    else if (this.dsq) return 'DSQ';
    else {
      let time = this.time!;
      if (paxMultiplier) time *= paxMultiplier;
      if (displayConeCount)
        return `${time.toFixed(3)}` + (this.cones ? ` (${this.cones})` : '');
      else return time.toFixed(3);
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

export class Driver {
  readonly id: string;
  readonly name: string;
  readonly carNumber: number;
  readonly carClass: string;
  readonly carDescription: string;
  readonly region: string;
  readonly times: LapTime[];
  readonly trophy: boolean;
  readonly rookie: boolean;
  readonly position: number;
  readonly dsq: boolean;

  constructor(
    carClass: string,
    [
      trophy,
      rookie,
      position,
      carNumber,
      name,
      carDescription,
      tireManufacturer,
      region,
    ]: string[],
    times: string[],
    fastest: string,
  ) {
    this.id = name.toLowerCase().trim(); // FIXME
    this.trophy = trophy === 'T';
    this.rookie = rookie === 'R';
    this.position = parseFloat(position);
    this.carNumber = parseInt(carNumber);
    this.carClass = carClass;
    this.name = name;
    this.carDescription = carDescription;
    this.region = region;
    this.times = times
      .filter((lapTime) => !!lapTime.trim())
      .map((lapTime) => new LapTime(lapTime));
    this.dsq =
      this.times.length !== 0 && fastest.trim().toLowerCase() === 'no time';
  }

  bestLap(): LapTime {
    if (this.dsq) return LapTime.DSQ;
    else return [...this.times].sort(LapTime.compare)[0];
  }

  difference(fastestOfDay?: number, paxMultiplier: number = 1): string {
    const myBestLap = this.bestLap();
    const myTimeToCompare = paxMultiplier * (myBestLap.time || Infinity);
    return myBestLap.time
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

  getBestInClass(): number | undefined {
    return [...this.drivers[0].times].sort(LapTime.compare)[0].time;
  }
}

export type ClassCategoryResults = Record<string, ClassResults>;

export type EventResults = Record<string, ClassCategoryResults>;

export type IndexedChampionshipType = 'PAX' | 'Novice' | 'Ladies';
export type ChampionshipType = 'Class' | IndexedChampionshipType;

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
