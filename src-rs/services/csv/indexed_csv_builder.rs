use crate::models::championship_driver::ChampionshipDriver;
use crate::models::championship_results::IndexedChampionshipResults;
use crate::models::championship_type::ChampionshipType;
use crate::services::trophy_calculator::{IndexTrophyCalculator, TrophyCalculator};
use crate::utilities::events_to_count;

pub trait IndexedCsvBuilder {
    fn create(
        &self,
        championship_type: ChampionshipType,
        results: IndexedChampionshipResults,
    ) -> Result<Option<String>, String>;
}

pub struct DefaultIndexedCsvBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
}

impl IndexedCsvBuilder for DefaultIndexedCsvBuilder {
    fn create(
        &self,
        championship_type: ChampionshipType,
        results: IndexedChampionshipResults,
    ) -> Result<Option<String>, String> {
        let event_count = results
            .drivers
            .get(0)
            .ok_or("Expected at least one driver")?
            .event_count();
        let events_to_count = events_to_count(event_count);
        let header = Self::build_header(event_count);
        let trophy_count = self.trophy_calculator.calculate(results.drivers.len());

        let mut rows = vec![
            results.organization.clone(),
            format!(
                "{} {} Championship -- Best {} of {} Events",
                results.year,
                championship_type.name(),
                events_to_count,
                event_count
            ),
            "".to_string(),
            header,
        ];
        let mut sorted = results.drivers.clone();
        sorted.sort_by_key(|d| d.best_of(events_to_count));
        sorted.reverse();
        rows.extend(
            sorted
                .iter()
                .enumerate()
                .filter(|(_, d)| d.total_points() != 0)
                .map(|(index, d)| {
                    let mut driver_row = vec![
                        if index < trophy_count {
                            "T".to_string()
                        } else {
                            "".to_string()
                        },
                        format!("{}", index + 1),
                        d.name().clone(),
                    ];
                    d.points()
                        .iter()
                        .for_each(|points| driver_row.push(format!("{}", points)));
                    driver_row.push(format!("{}", d.total_points()));
                    driver_row.push(format!("{}", d.best_of(events_to_count)));
                    driver_row.join(",")
                }),
        );

        Ok(Some(rows.join("\n")))
    }
}

impl DefaultIndexedCsvBuilder {
    pub fn new() -> DefaultIndexedCsvBuilder {
        DefaultIndexedCsvBuilder::from(None)
    }

    pub fn from(trophy_calculator: Option<Box<dyn TrophyCalculator>>) -> DefaultIndexedCsvBuilder {
        DefaultIndexedCsvBuilder {
            trophy_calculator: trophy_calculator.unwrap_or(Box::new(IndexTrophyCalculator {})),
        }
    }

    fn build_header(event_count: usize) -> String {
        let mut header = vec![
            "Trophy".to_string(),
            "Rank".to_string(),
            "Driver".to_string(),
        ];
        header.extend((0..event_count).map(|i| format!("Event #{}", i + 1)));
        header.push("Total Points".to_string());
        header.push(format!(
            "Best {} of {}",
            events_to_count(event_count),
            event_count
        ));
        header.join(",")
    }
}
