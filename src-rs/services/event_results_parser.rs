use std::collections::HashMap;
use std::num::{ParseFloatError, ParseIntError};

use csv::{StringRecord, Trim};

use crate::models::class_results::ClassResults;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::exported_driver::ExportedDriver;
use crate::models::lap_time::{LapTime, Penalty};
use crate::models::type_aliases::PaxMultiplier;
use crate::utilities::swap;

pub fn parse(file_contents: String, two_day_event: bool) -> Result<EventResults, String> {
    let mut reader1 = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(file_contents.as_bytes());

    let mut string_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(file_contents.as_bytes());
    let headers = string_reader.headers().map_err(|e| e.to_string())?;
    let header_vec: Vec<&str> = headers.iter().collect();
    let final_column_index = header_vec
        .binary_search(&"Runs Day2")
        .map_err(|_| String::from("Unable to find header column `Runs Day2`"))?;

    let mut results = HashMap::new();

    let records = reader1.deserialize().zip(string_reader.records());
    for (deserialized, string_rec) in records {
        let driver = perform_second_parsing(deserialized, string_rec, final_column_index + 1)?;
        let driver = Driver::from(driver, two_day_event);
        let class = driver.car_class.short;

        if !results.contains_key(&class) {
            results.insert(class, ClassResults::new(class));
        }

        results.get_mut(&class).map(|r| r.add_driver(driver));
    }

    Ok(EventResults { results })
}

fn perform_second_parsing(
    deserialized: csv::Result<ExportedDriver>,
    string_rec: csv::Result<StringRecord>,
    first_time_column: usize,
) -> Result<ExportedDriver, String> {
    let string_record = string_rec.map_err(|e| e.to_string())?;
    let strings_vec: Vec<&str> = string_record.iter().collect();

    let mut driver: ExportedDriver = deserialized.map_err(|e| e.to_string())?;

    let extra_fields = &strings_vec[first_time_column..];
    driver.day1 = swap(
        driver
            .runs_day1
            .map(|run_count| extract_lap_times(extra_fields, driver.pax_multiplier, run_count)),
    )?;

    if extra_fields.len() > driver.runs_day1.clone().unwrap_or(0) * 3 {
        driver.day2 = swap(driver.runs_day2.map(|run_count| {
            extract_lap_times(
                &extra_fields[(driver.runs_day1.clone().unwrap_or(0) * 3)..],
                driver.pax_multiplier,
                run_count,
            )
        }))?;
    } else {
        driver.day2 = None;
    }

    Ok(driver)
}

fn extract_lap_times(
    lap_time_fields: &[&str],
    pax: PaxMultiplier,
    run_count: usize,
) -> Result<Vec<LapTime>, String> {
    let mut times = Vec::new();
    for lap_number in 0..run_count {
        let first_index_of_lap = 3 * lap_number as usize;
        if lap_time_fields.len() < first_index_of_lap + 3 {
            break;
        }
        let next_fields = &lap_time_fields[first_index_of_lap..(first_index_of_lap + 3)];
        times.push(build_lap_time(next_fields, pax)?)
    }
    Ok(times)
}

fn build_lap_time(next_fields: &[&str], pax: PaxMultiplier) -> Result<LapTime, String> {
    Ok(LapTime::new(
        next_fields[0]
            .parse()
            .map_err(|e: ParseFloatError| e.to_string())?,
        pax,
        next_fields[1]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?,
        match next_fields[2] {
            "DNF" => Some(Penalty::DNF),
            "DNS" => Some(Penalty::DNS),
            "RRN" => Some(Penalty::RRN),
            "DSQ" => Some(Penalty::DSQ),
            _ => None,
        },
    ))
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::models::driver::TimeSelection;
    use crate::models::lap_time::{dns, LapTime, Penalty};
    use crate::models::short_car_class::ShortCarClass;
    use crate::services::event_results_parser::parse;

    #[test]
    fn parse_event_results() {
        let sample_contents =
            fs::read_to_string("./SampleData/2022_Event1-DavidExport.csv").unwrap();
        let actual = parse(sample_contents, false).unwrap();

        assert_eq!(actual.len(), 29);
        assert!(actual.results.contains_key(&ShortCarClass::AS));
        assert!(actual.results.contains_key(&ShortCarClass::BS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMC));
        assert!(actual.results.contains_key(&ShortCarClass::CAMS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMT));
        assert!(actual.results.contains_key(&ShortCarClass::CS));
        assert!(actual.results.contains_key(&ShortCarClass::CSP));
        assert!(actual.results.contains_key(&ShortCarClass::DP));
        assert!(actual.results.contains_key(&ShortCarClass::DS));
        assert!(actual.results.contains_key(&ShortCarClass::ES));
        assert!(actual.results.contains_key(&ShortCarClass::EVX));
        assert!(actual.results.contains_key(&ShortCarClass::FS));
        assert!(actual.results.contains_key(&ShortCarClass::FSAE));
        assert!(actual.results.contains_key(&ShortCarClass::FSP));
        assert!(actual.results.contains_key(&ShortCarClass::FUN));
        assert!(actual.results.contains_key(&ShortCarClass::GS));
        assert!(actual.results.contains_key(&ShortCarClass::GSL));
        assert!(actual.results.contains_key(&ShortCarClass::HS));
        assert!(actual.results.contains_key(&ShortCarClass::HSL));
        assert!(actual.results.contains_key(&ShortCarClass::SMF));
        assert!(actual.results.contains_key(&ShortCarClass::SSC));
        assert!(actual.results.contains_key(&ShortCarClass::STH));
        assert!(actual.results.contains_key(&ShortCarClass::STR));
        assert!(actual.results.contains_key(&ShortCarClass::STS));
        assert!(actual.results.contains_key(&ShortCarClass::STU));
        assert!(actual.results.contains_key(&ShortCarClass::STX));
        assert!(actual.results.contains_key(&ShortCarClass::XP));
        assert!(actual.results.contains_key(&ShortCarClass::XSA));
        assert!(actual.results.contains_key(&ShortCarClass::XSB));

        let a_street = actual.results.get(&ShortCarClass::AS).unwrap();
        assert_eq!(a_street.car_class.short, ShortCarClass::AS);
        assert_eq!(a_street.drivers.len(), 5);
        assert_eq!(
            a_street.get_best_in_class(None),
            LapTime::new(52.288, 0.821, 0, None)
        );
        assert_eq!(a_street.get_best_in_class(Some(TimeSelection::Day2)), dns());
        assert_eq!(a_street.get_best_in_class(Some(TimeSelection::Day2)), dns());

        let robert = a_street.drivers[0].clone();
        assert_eq!(robert.error, false);
        assert_eq!(robert.id, "robert fullriede");
        assert_eq!(robert.name, "Robert Fullriede");
        assert_eq!(robert.car_number, 52);
        assert_eq!(robert.car_class.short, ShortCarClass::AS);
        assert_eq!(robert.car_description, "2010 Porsche Cayman");
        assert_eq!(robert.region, "");
        assert_eq!(robert.rookie, false);
        assert_eq!(robert.ladies_championship, false);
        assert_eq!(robert.position, Some(1));
        assert_eq!(robert.dsq, false);
        assert_eq!(robert.pax_multiplier, 0.821);
        assert_eq!(
            robert.day_1_times,
            Some(vec![
                LapTime::new(52.288, 0.821, 0, None),
                LapTime::new(53.351, 0.821, 0, None),
                LapTime::new(0., 0.821, 0, Some(Penalty::DNF)),
            ])
        );
        assert_eq!(robert.day_2_times, None);
        assert_eq!(robert.combined, LapTime::new(52.288, 0.821, 0, None));

        for (index, driver) in a_street.drivers.iter().enumerate() {
            assert_eq!(driver.position, Some(index + 1));
        }
    }
}
