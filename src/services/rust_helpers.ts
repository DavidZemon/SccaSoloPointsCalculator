import {
  EventResults as RustyEventResults,
  ClassResults as RustyClassResults,
  Driver as RustyDriver,
  LapTime,
  DriverGroup,
  ShortCarClass,
} from 'rusty/rusty';

export type EventResults = Exclude<
  RustyEventResults,
  | 'js_drivers_in_error'
  | 'get_js_drivers'
  | 'get_all_driver_js_names'
  | 'get_all_js'
  | 'get_js'
> & {
  get_drivers: (filter?: DriverGroup) => Driver[];
  drivers_in_error: () => string[];
  get_all_driver_names: () => string[];
  get_all: () => ClassResults[];
  get: (carClass: ShortCarClass) => ClassResults | undefined;
};

export type ClassResults = Exclude<RustyClassResults, 'get_drivers'> & {
  drivers: Driver[];
};

export type Driver = Exclude<
  RustyDriver,
  'get_day_1_times' | 'get_day_2_times'
> & {
  day_1_times?: LapTime[];
  day_2_times?: LapTime[];
};

export function convertEventResults(rusty: RustyEventResults): EventResults {
  const results = rusty as EventResults;
  results.get_drivers = (filter) =>
    rusty.get_js_drivers(filter).map((d) => convertDriver(new RustyDriver(d)));
  results.drivers_in_error = () => rusty.js_drivers_in_error() as string[];
  results.get_all_driver_names = () =>
    rusty.get_all_driver_js_names() as string[];
  results.get_all = () =>
    rusty
      .get_all_js()
      .map((r: any) => convertClassResults(new RustyClassResults(r))!)!;
  results.get = (carClass) => convertClassResults(rusty.get_js(carClass));
  return results;
}

export function convertClassResults(
  rusty?: RustyClassResults,
): ClassResults | undefined {
  if (rusty) {
    const results = rusty as ClassResults;
    results.drivers = rusty
      .get_drivers()
      .map((d) => convertDriver(new RustyDriver(d)));
    return results;
  }
}

export function convertDriver(rusty: RustyDriver): Driver {
  const driver = rusty as Driver;
  driver.day_1_times = rusty.get_day_1_times()?.map((t: any) => new LapTime(t));
  driver.day_2_times = rusty.get_day_2_times()?.map((t: any) => new LapTime(t));
  return driver;
}
