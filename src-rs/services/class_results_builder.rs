use csv::Writer;
use wasm_bindgen::JsValue;

use crate::models::car_class::{get_car_class, CarClass};
use crate::models::class_results::ClassResults;
use crate::models::event_results::EventResults;
use crate::models::type_aliases::Time;
use crate::services::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};
use crate::services::trophy_calculator::{ClassTrophyCalculator, TrophyCalculator};

pub struct ClassResultsBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl ClassResultsBuilder {
    pub fn new() -> ClassResultsBuilder {
        ClassResultsBuilder::from(None, None)
    }

    fn from(
        trophy_calculator: Option<Box<dyn TrophyCalculator>>,
        points_calculator: Option<Box<dyn ChampionshipPointsCalculator>>,
    ) -> ClassResultsBuilder {
        ClassResultsBuilder {
            trophy_calculator: trophy_calculator.unwrap_or(Box::new(ClassTrophyCalculator {})),
            points_calculator: points_calculator
                .unwrap_or(Box::new(DefaultChampionshipPointsCalculator {})),
        }
    }

    pub fn to_csvs(&self, results: &EventResults) -> Vec<JsValue> {
        let mut results = results
            .results
            .iter()
            .map(|(class, results)| {
                (
                    get_car_class(class)
                        .expect(format!("Missing class {} in class map", class.name()).as_str()),
                    results,
                )
            })
            .collect::<Vec<(CarClass, &ClassResults)>>();

        results.sort_by(|(lhs, ..), (rhs, ..)| {
            if lhs.category == rhs.category {
                lhs.short.cmp(&rhs.short)
            } else {
                lhs.category.cmp(&rhs.category)
            }
        });

        results
            .iter()
            .map(|(class, results)| {
                JsValue::from_serde(&(class, self.export_class(results))).expect(
                    format!("Failed to serialize class CSV for {}", class.long.name()).as_str(),
                )
            })
            .collect()
    }

    pub fn get_header(&self) -> String {
        vec![
            "Trophy".to_string(),
            "Pos".to_string(),
            "Name".to_string(),
            "Car".to_string(),
            "Class".to_string(),
            "Number".to_string(),
            "Total Time".to_string(),
            "Index".to_string(),
            "From Previous".to_string(),
            "From Top".to_string(),
            "Points".to_string(),
        ]
        .join(",")
    }

    fn export_class(&self, class_results: &ClassResults) -> String {
        let short_class_name = class_results.car_class.short.name().to_string();
        let trophy_count = self
            .trophy_calculator
            .calculate(class_results.drivers.len());

        let mut csv = Writer::from_writer(vec![]);

        let best_lap_in_class = class_results.get_best_in_class(None);
        let best_index_time = best_lap_in_class
            * class_results
                .drivers
                .get(0)
                .expect(
                    format!(
                        "Class results for {} contain no drivers",
                        short_class_name.clone()
                    )
                    .as_str(),
                )
                .pax_multiplier;

        class_results.drivers.iter().enumerate().for_each(|(i, d)| {
            csv.write_record(vec![
                if i < trophy_count {
                    "T".to_string()
                } else {
                    "".to_string()
                },
                d.position
                    .map(|p| format!("{}", p))
                    .unwrap_or("".to_string()),
                d.name.clone(),
                d.car_description.clone(),
                short_class_name.clone(),
                format!("{}", d.car_number),
                d.best_lap(None).to_string(None, Some(false)),
                d.best_lap(None)
                    .to_string(Some(d.pax_multiplier), Some(false)),
                if i == 0 {
                    "".to_string()
                } else {
                    d.difference(
                        class_results
                            .drivers
                            .get(i - 1)
                            .unwrap()
                            .best_lap(None)
                            .time
                            .unwrap_or(Time::INFINITY)
                            * d.pax_multiplier,
                        Some(true),
                        None,
                    )
                },
                d.difference(best_index_time, Some(true), None),
                format!(
                    "{}",
                    self.points_calculator
                        .calculate(best_index_time, d, Some(d.pax_multiplier))
                ),
            ])
            .expect(format!("Failed to write record for {} to class results CSV", d.name).as_str());
        });

        String::from_utf8(csv.into_inner().unwrap()).unwrap()
    }
}
