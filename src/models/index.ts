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
        const time_parts = lap_time_str.split('(');
        this.time = parseFloat(time_parts[0]);
        this.cones = time_parts.length === 1 ? parseInt(time_parts[1]) : 0;
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

export interface IndividualResults {
  id: string;
  name: string;
  carNumber: number;
  carClass: string;
  carDescription: string;
  times: LapTime[];
  trophy: boolean;
  rookie: boolean;
  position: number;
}

export class ClassResults {
  public trophyCount: number;
  public readonly carClass: string;
  public readonly results: IndividualResults[];

  constructor(carClass: string) {
    this.trophyCount = 0;
    this.carClass = carClass;
    this.results = [];
  }
}

export type ClassCategoryResults = Record<string, ClassResults>;

export type EventResults = Record<string, ClassCategoryResults>;
