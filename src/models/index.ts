export class LapTime {
  public static readonly DSQ = new LapTime('DSQ');

  public readonly time?: number;
  public readonly cones: number;
  public readonly dnf: boolean;
  public readonly rerun: boolean;
  public readonly dsq: boolean;
  public readonly dns: boolean;

  constructor(lap_time_str: string) {
    switch (lap_time_str) {
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
      return new LapTime(`${this.time! + rhs.time!}`);
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
  readonly day1Times: LapTime[];
  readonly day2Times: LapTime[];
  readonly combined: LapTime;
  readonly trophy: boolean;
  readonly rookie: boolean;
  position: number;
  readonly dsq: boolean;

  constructor(
    carClass: string,
    [
      rookie,
      trophy,
      position,
      carNumber,
      _1,
      name,
      carDescription,
      _2,
    ]: string[],
    day1Times: string[],
    day2Times: string[],
  ) {
    this.id = name.toLowerCase().trim(); // FIXME
    this.trophy = trophy === 'T';
    this.rookie = rookie === 'M';
    this.position = parseFloat(position);
    this.carNumber = parseInt(carNumber);
    this.carClass = carClass;
    this.name = name;
    this.carDescription = carDescription;
    this.region = '';
    this.day1Times = day1Times
      .filter((lapTime) => !!lapTime.trim())
      .map((lapTime) => new LapTime(lapTime));
    this.day2Times = day2Times
      .filter((lapTime) => !!lapTime.trim())
      .map((lapTime) => new LapTime(lapTime));
    this.dsq = false;
    this.combined = this.bestLap(this.day1Times).add(
      this.bestLap(this.day2Times),
    );
  }

  bestLap(times: LapTime[]): LapTime {
    if (this.dsq) return LapTime.DSQ;
    else return [...times].sort(LapTime.compare)[0];
  }

  difference(fastestOfDay?: number, paxMultiplier: number = 1): string {
    const timeToCompare = this.bestLap(this.day1Times);
    const myTimeToCompare = paxMultiplier * (timeToCompare.time || Infinity);
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

  getBestInClass(): number | undefined {
    return this.drivers[0].bestLap(this.drivers[0].day1Times).time;
  }
}

export type EventResults = Record<string, ClassResults>;

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
