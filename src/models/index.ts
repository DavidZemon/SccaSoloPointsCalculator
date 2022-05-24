import {ShortCarClass, CarClass, LongCarClass, ClassCategory} from 'rusty/rusty';

export type IndexedChampionshipType = 'PAX' | 'Novice' | 'Ladies';
export type ChampionshipType = 'Class' | IndexedChampionshipType;

export type MangledCarClass = Omit<CarClass, 'short' | 'long' | 'category'> & {
  short: keyof typeof ShortCarClass;
  long: keyof typeof LongCarClass;
  category: keyof typeof ClassCategory;
};

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
  driversByClass: Record<ShortCarClass, ClassChampionshipDriver[]>;
}

export interface ChampionshipResults {
  Class?: ClassChampionshipResults;
  PAX?: IndexedChampionshipResults;
  Novice?: IndexedChampionshipResults;
  Ladies?: IndexedChampionshipResults;
}
