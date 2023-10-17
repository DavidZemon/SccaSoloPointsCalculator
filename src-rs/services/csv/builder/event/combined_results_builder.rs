use crate::enums::championship_type::ChampionshipType;
use csv::Writer;

use crate::enums::driver_group::DriverGroup;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::lap_time::LapTime;
use crate::services::calculators::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};
use crate::services::calculators::tie_calculator::calculate_tie_offset;
use crate::services::calculators::trophy_calculator::{DefaultTrophyCalculator, TrophyCalculator};

/// Build indexed and raw time CSV results for a single event
pub struct CombinedResultsBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl Default for CombinedResultsBuilder {
    fn default() -> Self {
        CombinedResultsBuilder::from(None, None)
    }
}

impl CombinedResultsBuilder {
    pub fn to_combined_csv(
        &self,
        results: &EventResults,
        driver_group: DriverGroup,
    ) -> Result<String, String> {
        let is_raw_time = driver_group == DriverGroup::Raw;

        let drivers = results.get_drivers(Some(driver_group));
        if drivers.is_empty() {
            Ok(format!("No drivers for {} group", driver_group.name()))
        } else {
            let csv = self.build_csv(drivers, driver_group, is_raw_time)?;
            let csv_byte_array = csv.into_inner().map_err(|e| e.to_string())?;
            String::from_utf8(csv_byte_array).map_err(|e| e.to_string())
        }
    }

    fn from(
        trophy_calculator: Option<Box<dyn TrophyCalculator>>,
        points_calculator: Option<Box<dyn ChampionshipPointsCalculator>>,
    ) -> Self {
        CombinedResultsBuilder {
            trophy_calculator: trophy_calculator
                .unwrap_or_else(|| Box::new(DefaultTrophyCalculator {})),
            points_calculator: points_calculator
                .unwrap_or_else(|| Box::new(DefaultChampionshipPointsCalculator {})),
        }
    }

    fn get_combined_header(&self, is_raw_time: bool) -> Vec<String> {
        let mut time_column = (if is_raw_time { "Best" } else { "Index" }).to_string();
        time_column.push_str(" Time");
        let mut header = vec![
            "Trophy".to_string(),
            "Position".to_string(),
            "Name".to_string(),
            "Car".to_string(),
            "Class".to_string(),
            "Car #".to_string(),
            time_column,
            "From Previous".to_string(),
            "From Top".to_string(),
        ];
        if !is_raw_time {
            header.push("Points".to_string());
        }
        header
    }

    fn build_csv(
        &self,
        drivers: Vec<&Driver>,
        driver_group: DriverGroup,
        is_raw_time: bool,
    ) -> Result<Writer<Vec<u8>>, String> {
        let fastest_driver = drivers.get(0).unwrap();
        let fastest_of_day = fastest_driver.best_lap(None);

        let driver_count = drivers.len();
        let trophy_count = self
            .trophy_calculator
            .calculate(driver_count, ChampionshipType::from(driver_group));

        let mut csv = Writer::from_writer(vec![]);
        csv.write_record(self.get_combined_header(is_raw_time))
            .map_err(|e| e.to_string())?;

        for i in 0..driver_count {
            let next_row = self.build_record(
                i,
                &drivers,
                driver_group,
                trophy_count,
                is_raw_time,
                fastest_of_day,
            )?;
            csv.write_record(next_row).map_err(|e| e.to_string())?;
        }

        Ok(csv)
    }

    fn build_record(
        &self,
        i: usize,
        drivers: &[&Driver],
        driver_group: DriverGroup,
        trophy_count: usize,
        is_raw_time: bool,
        fastest_of_day: LapTime,
    ) -> Result<Vec<String>, String> {
        let previous_driver = drivers.get(i - 1);
        let driver = drivers.get(i).map_or(
            Err(format!(
                "expected at least one driver for {}",
                driver_group.name()
            )),
            Ok,
        )?;

        let tie_offset =
            calculate_tie_offset(drivers, i, |d1, d2| d1.best_lap(None) == d2.best_lap(None));

        let mut next_row = vec![
            if (i - tie_offset) < trophy_count {
                "T".to_string()
            } else {
                "".to_string()
            },
            format!("{}", i + 1 - tie_offset),
            driver.name.clone(),
            driver.car_description.clone(),
            driver.car_class.short.name().to_string(),
            format!("{}", driver.car_number),
            driver.best_lap(None).to_string(!is_raw_time, false),
            previous_driver
                .map(|prev| driver.difference(prev.best_lap(None), !is_raw_time, None))
                .unwrap_or_else(|| "".to_string()),
            driver.difference(fastest_of_day, !is_raw_time, None),
        ];
        if !is_raw_time {
            next_row.push(format!(
                "{}",
                self.points_calculator.calculate(&fastest_of_day, driver,)
            ))
        }

        Ok(next_row)
    }
}
