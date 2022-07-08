use csv::Writer;

use crate::models::driver_group::DriverGroup;
use crate::models::event_results::EventResults;
use crate::models::type_aliases::Time;
use crate::services::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};
use crate::services::trophy_calculator::{IndexTrophyCalculator, TrophyCalculator};

pub struct CombinedResultsBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl CombinedResultsBuilder {
    pub fn new() -> CombinedResultsBuilder {
        CombinedResultsBuilder::from(None, None)
    }

    pub fn to_combined_csv(
        &self,
        results: &EventResults,
        driver_group: DriverGroup,
    ) -> Result<String, String> {
        let is_raw_time = driver_group == DriverGroup::Raw;

        let drivers = results.get_drivers(Some(driver_group));
        if 0 == drivers.len() {
            Ok(format!("No drivers for {} group", driver_group.name()))
        } else {
            let fastest_driver = drivers.get(0).unwrap();
            let fastest_of_day = fastest_driver.best_lap(None).with_pax(match driver_group {
                DriverGroup::Raw => 1.,
                _ => fastest_driver.pax_multiplier,
            });

            let driver_count = drivers.len();
            let trophy_count = self.trophy_calculator.calculate(driver_count);

            let mut csv = Writer::from_writer(vec![]);
            csv.write_record(self.get_combined_header(is_raw_time))
                .map_err(|e| e.to_string())?;

            for i in 0..driver_count {
                let previous_driver = drivers.get(i - 1);
                let driver = drivers.get(i).map_or(
                    Err(format!(
                        "expected at least one driver for {}",
                        driver_group.name()
                    )),
                    |d| Ok(d),
                )?;

                let mut next_row = vec![
                    String::from(if i < trophy_count { "T" } else { "" }),
                    format!("{}", i + 1),
                    driver.name.clone(),
                    driver.car_description.clone(),
                    String::from(driver.car_class.short.name()),
                    format!("{}", driver.car_number),
                    driver.best_lap(None).to_string(
                        if is_raw_time {
                            None
                        } else {
                            Some(driver.pax_multiplier)
                        },
                        Some(false),
                    ),
                    match previous_driver {
                        None => String::from(""),
                        Some(prev) => {
                            let use_pax = !is_raw_time;
                            let previous_raw_time =
                                prev.best_lap(None).time.unwrap_or(Time::INFINITY);
                            driver.difference(
                                if use_pax {
                                    previous_raw_time * prev.pax_multiplier
                                } else {
                                    previous_raw_time
                                },
                                Some(use_pax),
                                None,
                            )
                        }
                    },
                    driver.difference(fastest_of_day, Some(!is_raw_time), None),
                ];
                if !is_raw_time {
                    next_row.push(format!(
                        "{}",
                        self.points_calculator.calculate(
                            fastest_of_day,
                            driver,
                            Some(driver.pax_multiplier)
                        )
                    ))
                }
                csv.write_record(next_row).map_err(|e| e.to_string())?;
            }

            let csv_byte_array = csv.into_inner().map_err(|e| e.to_string())?;
            String::from_utf8(csv_byte_array).map_err(|e| e.to_string())
        }
    }

    fn from(
        trophy_calculator: Option<Box<dyn TrophyCalculator>>,
        points_calculator: Option<Box<dyn ChampionshipPointsCalculator>>,
    ) -> CombinedResultsBuilder {
        CombinedResultsBuilder {
            trophy_calculator: trophy_calculator.unwrap_or(Box::new(IndexTrophyCalculator {})),
            points_calculator: points_calculator
                .unwrap_or(Box::new(DefaultChampionshipPointsCalculator {})),
        }
    }

    fn get_combined_header(&self, is_raw_time: bool) -> Vec<String> {
        let mut time_column = String::from(if is_raw_time { "Best" } else { "Index" });
        time_column.push_str(" Time");
        let mut header = vec![
            String::from("Trophy"),
            String::from("Position"),
            String::from("Name"),
            String::from("Car"),
            String::from("Class"),
            String::from("Car #"),
            time_column,
            String::from("From Previous"),
            String::from("From Top"),
        ];
        if !is_raw_time {
            header.push(String::from("Points"));
        }
        header
    }
}
