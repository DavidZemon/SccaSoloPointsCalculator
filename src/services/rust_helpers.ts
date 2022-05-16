import {
  ClassResults as RustyClassResults,
  Driver as RustyDriver,
  LapTime,
} from 'rusty/rusty';

export type Driver = RustyDriver & {
  day_1_times?: LapTime[];
  day_2_times?: LapTime[];
};

export type ClassResults = Exclude<RustyClassResults, 'get_drivers'> & {
  drivers: Driver[];
};

export function convertClassResults(
  results?: RustyClassResults,
): ClassResults | undefined {
  if (results)
    return {
      ...results,
      drivers: results
        .get_drivers()
        .map((d) => JSON.stringify(d))
        .map((d: any) => new RustyDriver(d))
        .map((d) => convertDriver(d)),
    } as ClassResults;
}

export function convertDriver(driver: Driver): Driver {
  driver.day_1_times = driver
    .get_day_1_times()
    ?.map((t: any) => new LapTime(t));
  driver.day_2_times = driver
    .get_day_2_times()
    ?.map((t: any) => new LapTime(t));
  return driver;
}
