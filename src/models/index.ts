export class LapTime {
  public readonly time?: number;
  public readonly cones: number;
  public readonly dnf: boolean;
  public readonly rerun: boolean;

  constructor(lap_time_str: string) {
    switch (lap_time_str) {
      case 'DNF':
        this.dnf = true;
        this.rerun = false;
        this.cones = 0;
        break;
      case 'RRN':
        this.dnf = false;
        this.rerun = true;
        this.cones = 0;
        break;
      default:
        this.dnf = false;
        this.rerun = false;
        const timeParts = lap_time_str.split('(');
        this.time = parseFloat(timeParts[0]);
        this.cones =
          timeParts.length === 2
            ? parseInt(timeParts[1].slice(0, timeParts[1].length - 1))
            : 0;
    }
  }

  toString(): string {
    if (this.dnf) return 'DNF';
    else if (this.rerun) return 'Re-run';
    else return `${this.time}` + (this.cones ? ` (${this.cones})` : '');
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
  readonly times: LapTime[];
  readonly trophy: boolean;
  readonly rookie: boolean;
  readonly position: number;

  constructor(
    carClass: string,
    [trophy, rookie, position, carNumber, name, carDescription]: string[],
    times: string[],
  ) {
    this.id = 'N/A'; // FIXME
    this.trophy = trophy === 'T';
    this.rookie = rookie === 'R';
    this.position = parseFloat(position);
    this.carNumber = parseInt(carNumber);
    this.carClass = carClass;
    this.name = name;
    this.carDescription = carDescription;
    this.times = times
      .filter((lapTime) => !!lapTime.trim())
      .map((lapTime) => new LapTime(lapTime));
  }

  bestLap(): LapTime {
    return [...this.times].sort(LapTime.compare)[0];
  }

  difference(bestLapInClass?: number): string {
    const myBestLap = this.bestLap();
    return bestLapInClass === myBestLap.time
      ? ''
      : `(${(bestLapInClass! - myBestLap.time!).toFixed(3)})`;
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
