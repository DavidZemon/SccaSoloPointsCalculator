#[cfg(test)]
use mockall::automock;

use crate::enums::long_car_class::to_display_name;
use crate::enums::short_car_class::ShortCarClass;
use crate::models::car_class::get_car_class;
use crate::models::championship_driver::ChampionshipDriver;
use crate::models::championship_results::ClassChampionshipResults;
use crate::services::calculators::tie_calculator::calculate_tie_offset;
use crate::services::calculators::trophy_calculator::{ClassTrophyCalculator, TrophyCalculator};
use crate::utilities::events_to_count;

#[cfg_attr(test, automock)]
pub trait ClassCsvBuilder {
    fn create(&self, class: ClassChampionshipResults) -> Result<Option<String>, String>;
}

pub struct DefaultClassCsvBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
}

impl ClassCsvBuilder for DefaultClassCsvBuilder {
    fn create(&self, results: ClassChampionshipResults) -> Result<Option<String>, String> {
        let event_count = results
            .drivers_by_class
            .values()
            .next()
            .ok_or("Expected at least one class")?
            .get(0)
            .ok_or("Expected at least one driver in at least one class")?
            .event_count();
        let events_to_count = events_to_count(event_count);
        let header = Self::build_header(events_to_count, event_count);

        let mut rows = vec![
            results.organization.clone(),
            format!(
                "{} Class Championship -- Best {} of {} Events",
                results.year, events_to_count, event_count
            ),
            "".to_string(),
            header,
        ];

        let mut sorted = results
            .drivers_by_class
            .iter()
            .map(|(k, v)| {
                let mut v = v.clone();
                v.sort_by(|lhs, rhs| {
                    rhs.best_of(events_to_count)
                        .cmp(&lhs.best_of(events_to_count))
                });
                (k.clone(), v)
            })
            .collect::<Vec<(ShortCarClass, Vec<ChampionshipDriver>)>>();
        sorted.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

        sorted.iter().for_each(|(class, drivers)| {
            rows.push(format!(
                "{} - {}",
                class.name(),
                to_display_name(get_car_class(class).unwrap().long)
            ));

            let trophy_count = self.trophy_calculator.calculate(drivers.len());
            rows.extend(drivers.iter().enumerate().map(|(index, d)| {
                let tie_offset = calculate_tie_offset(&drivers, index, |d1, d2| {
                    d1.total_points() == d2.total_points()
                });

                let mut driver_row = vec![
                    if (index - tie_offset) < trophy_count {
                        "T".to_string()
                    } else {
                        "".to_string()
                    },
                    format!("{}", index + 1 - tie_offset),
                    d.name().clone(),
                ];
                d.points()
                    .iter()
                    .for_each(|points| driver_row.push(format!("{}", points)));
                driver_row.push(format!("{}", d.total_points()));
                driver_row.push(format!("{}", d.best_of(events_to_count)));
                driver_row.join(",")
            }))
        });

        Ok(Some(rows.join("\n")))
    }
}

impl DefaultClassCsvBuilder {
    pub fn new() -> DefaultClassCsvBuilder {
        DefaultClassCsvBuilder::from(None)
    }

    pub fn from(trophy_calculator: Option<Box<dyn TrophyCalculator>>) -> DefaultClassCsvBuilder {
        DefaultClassCsvBuilder {
            trophy_calculator: trophy_calculator.unwrap_or(Box::new(ClassTrophyCalculator {})),
        }
    }

    fn build_header(events_to_count: usize, event_count: usize) -> String {
        let mut header = vec![
            "Trophy".to_string(),
            "Rank".to_string(),
            "Driver".to_string(),
        ];
        header.extend((0..event_count).map(|i| format!("Event #{}", i + 1)));
        header.push("Total Points".to_string());
        header.push(format!("Best {} of {}", events_to_count, event_count));

        header.join(",")
    }
}
