use crate::enums::championship_type::ChampionshipType;
use crate::enums::short_car_class::ShortCarClass;
use crate::models::car_class::{get_car_class, CarClass};
use crate::models::class_results::ClassResults;
use crate::models::event_results::EventResults;
use crate::services::calculators::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};
use crate::services::calculators::trophy_calculator::{DefaultTrophyCalculator, TrophyCalculator};
use csv::Writer;
use wasm_bindgen::JsValue;

/// Build class CSV results for a single event
pub struct ClassResultsBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl Default for ClassResultsBuilder {
    fn default() -> Self {
        Self::from(None, None)
    }
}

impl ClassResultsBuilder {
    fn from(
        trophy_calculator: Option<Box<dyn TrophyCalculator>>,
        points_calculator: Option<Box<dyn ChampionshipPointsCalculator>>,
    ) -> Self {
        Self {
            trophy_calculator: trophy_calculator.unwrap_or_else(|| Box::new(DefaultTrophyCalculator {})),
            points_calculator: points_calculator.unwrap_or_else(|| Box::new(DefaultChampionshipPointsCalculator {})),
        }
    }

    pub fn to_csvs(&self, results: &EventResults) -> Vec<JsValue> {
        let mut results = results
            .results
            .iter()
            .map(|(class, results)| {
                (
                    get_car_class(class).unwrap_or_else(|| panic!("Missing class {} in class map", class.name())),
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
                serde_wasm_bindgen::to_value(&(class, self.export_class(results)))
                    .unwrap_or_else(|_| panic!("Failed to serialize class CSV for {}", class.long.name()))
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
            .calculate(class_results.drivers.len(), Some(ChampionshipType::Class));

        let mut csv = Writer::from_writer(vec![]);

        let best_lap_in_class = class_results.get_best_in_class();

        class_results.drivers.iter().enumerate().for_each(|(i, d)| {
            let compare_on_pro = class_results.car_class.short == ShortCarClass::P;
            let best_lap = d.best_lap(compare_on_pro);
            csv.write_record(vec![
                if i < trophy_count {
                    "T".to_string()
                } else {
                    "".to_string()
                },
                d.position.map(|p| format!("{p}")).unwrap_or_default(),
                d.name.clone(),
                d.car_description.clone(),
                if compare_on_pro {
                    d.pax_class.map(|c| c.short.name()).unwrap_or_default().to_string()
                } else {
                    short_class_name.clone()
                },
                format!("{}", d.car_number),
                best_lap.to_string(false, false),
                best_lap.to_string(true, false),
                if i == 0 {
                    "".to_string()
                } else {
                    d.difference(
                        class_results.drivers.get(i - 1).unwrap().best_lap(compare_on_pro),
                        true,
                        compare_on_pro,
                    )
                },
                d.difference(best_lap_in_class.clone(), true, compare_on_pro),
                format!(
                    "{}",
                    self.points_calculator.calculate(&best_lap_in_class, d, compare_on_pro)
                ),
            ])
            .unwrap_or_else(|_| panic!("Failed to write record for {} to class results CSV", d.name));
        });

        String::from_utf8(csv.into_inner().unwrap()).unwrap()
    }
}
