use crate::models::driver_group::DriverGroup;
use csv::Writer;
use wasm_bindgen::prelude::*;

use crate::models::event_results::EventResults;
use crate::models::type_aliases::Time;
use crate::services::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};
use crate::services::trophy_calculator::{DefaultTrophyCalculator, TrophyCalculator};

#[wasm_bindgen]
pub struct CombinedResultsBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

#[wasm_bindgen]
impl CombinedResultsBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CombinedResultsBuilder {
        CombinedResultsBuilder::from(None, None)
    }

    pub fn to_combined_csv(&self, results: &EventResults, driver_group: DriverGroup) -> String {
        let is_raw_time = driver_group == DriverGroup::Raw;

        let drivers = results.get_drivers(Some(driver_group));
        let fastest_driver = drivers
            .get(0)
            .expect("Should have received at least one driver, but found none!");
        let fastest_of_day = fastest_driver.best_lap(None).with_pax(match driver_group {
            DriverGroup::Raw => 1.,
            _ => fastest_driver.pax_multiplier,
        });

        let driver_count = drivers.len();
        let trophy_count = self.trophy_calculator.calculate(driver_count);

        let mut csv = Writer::from_writer(vec![]);
        csv.write_record(self.get_combined_header(is_raw_time))
            .unwrap();

        for i in 0..driver_count {
            let previous_driver = drivers.get(i - 1);
            let driver = drivers.get(i).unwrap();

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
                    Some(prev) => driver.difference(
                        prev.best_lap(None).time.unwrap_or(Time::INFINITY),
                        Some(!is_raw_time),
                        None,
                    ),
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
            csv.write_record(next_row).unwrap();
        }

        String::from_utf8(csv.into_inner().unwrap()).unwrap()
    }
}

impl CombinedResultsBuilder {
    pub fn from(
        trophy_calculator: Option<Box<dyn TrophyCalculator>>,
        points_calculator: Option<Box<dyn ChampionshipPointsCalculator>>,
    ) -> CombinedResultsBuilder {
        CombinedResultsBuilder {
            trophy_calculator: trophy_calculator.unwrap_or(Box::new(DefaultTrophyCalculator {})),
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
