use std::collections::HashMap;
use std::num::{ParseFloatError, ParseIntError};

use csv::{StringRecord, Trim};

use crate::models::class_results::ClassResults;
use crate::models::driver::Driver;
use crate::models::driver_from_pronto::DriverFromPronto;
use crate::models::event_results::EventResults;
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
        .map_err(|_| "Unable to find header column `Runs Day2`".to_string())?;

    let mut results = HashMap::new();

    let records = reader1.deserialize().zip(string_reader.records());
    for (deserialized, string_rec) in records {
        let (driver, string_rec) = validate_row(deserialized, string_rec)?;

        let driver = perform_second_parsing(driver, string_rec, final_column_index + 1)?;
        let driver = Driver::from(driver, two_day_event);
        let class = driver.car_class.short;

        results
            .entry(class)
            .or_insert_with(|| ClassResults::new(class));

        if let Some(r) = results.get_mut(&class) {
            r.add_driver(driver)
        }
    }

    Ok(EventResults { results })
}

fn validate_row(
    deserialized: csv::Result<DriverFromPronto>,
    string_record: csv::Result<StringRecord>,
) -> Result<(DriverFromPronto, StringRecord), String> {
    let string_record = string_record.map_err(|e| e.to_string())?;

    match deserialized {
        Ok(driver) => Ok((driver, string_record)),
        Err(e) => match e.kind() {
            csv::ErrorKind::Deserialize { err: root, .. } => {
                match root.kind() {
                    csv::DeserializeErrorKind::UnexpectedEndOfRow => Err(
                        format!(
                            "Encountered an unexpected end of row for a record. One common reason for this is a driver that did not attend the event but remains in Pronto.\n'{:?}'",
                            string_record,
                        )
                    ),
                    _ => Err(format!("Failed to deserialize row {:?} due to {:?}", string_record, e.to_string())),
                }
            },
            _ => Err(format!("Failed to deserialize row {:?} due to {:?}", string_record, e.to_string())),
        },
    }
}

fn perform_second_parsing(
    mut driver: DriverFromPronto,
    string_record: StringRecord,
    first_time_column: usize,
) -> Result<DriverFromPronto, String> {
    // Start by just dealing with the Pronto's poor handling of our "Ladies" field in that it
    // sometimes (always?) puts the ladies indicator in the region column
    {
        if let Some(region) = driver.region.clone() {
            if region.trim() == "L" {
                driver.ladies = Some(region);
            }
        }
    }

    // Then deal with the day 1/2 lap times
    {
        let strings_vec: Vec<&str> = string_record.iter().collect();

        let extra_fields = &strings_vec[first_time_column..];
        driver.day1 =
            swap(driver.runs_day1.map(|run_count| {
                extract_lap_times(extra_fields, driver.pax_multiplier, run_count)
            }))?;

        if extra_fields.len() > driver.runs_day1.unwrap_or(0) * 3 {
            driver.day2 = swap(driver.runs_day2.map(|run_count| {
                extract_lap_times(
                    &extra_fields[(driver.runs_day1.unwrap_or(0) * 3)..],
                    driver.pax_multiplier,
                    run_count,
                )
            }))?;
        } else {
            driver.day2 = None;
        }
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

    use crate::enums::short_car_class::ShortCarClass;
    use crate::models::driver::TimeSelection;
    use crate::models::lap_time::{dns, LapTime, Penalty};
    use crate::services::csv::parser::event_results_parser::parse;

    #[test]
    fn parse_2022_e1_event_results() {
        let sample_contents =
            fs::read_to_string("./SampleData/2022_Event1-DavidExport.csv").unwrap();
        let actual = parse(sample_contents, false).unwrap();

        assert_eq!(actual.results.len(), 29);
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
        assert!(actual.results.contains_key(&ShortCarClass::XA));
        assert!(actual.results.contains_key(&ShortCarClass::XB));

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
        assert!(!robert.error);
        assert_eq!(robert.id, "robert fullriede");
        assert_eq!(robert.name, "Robert Fullriede");
        assert_eq!(robert.car_number, 52);
        assert_eq!(robert.car_class.short, ShortCarClass::AS);
        assert_eq!(robert.car_description, "2010 Porsche Cayman");
        assert_eq!(robert.region, "");
        assert!(!robert.rookie);
        assert!(!robert.ladies_championship);
        assert_eq!(robert.position, Some(1));
        assert!(!robert.dsq);
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

    #[test]
    fn parse_2023_e3_event_results() {
        let sample_contents =
            fs::read_to_string("./SampleData/2023_Event3-DavidExport.csv").unwrap();

        let actual = match parse(sample_contents, false) {
            Ok(actual) => actual,
            Err(e) => panic!("Parsed failed due to: {}", e),
        };
        assert!(actual.results.contains_key(&ShortCarClass::AS));
        assert!(actual.results.contains_key(&ShortCarClass::BS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMC));
        assert!(actual.results.contains_key(&ShortCarClass::CAMS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMT));
        assert!(actual.results.contains_key(&ShortCarClass::CS));
        assert!(actual.results.contains_key(&ShortCarClass::DM));
        assert!(actual.results.contains_key(&ShortCarClass::DML));
        assert!(actual.results.contains_key(&ShortCarClass::DS));
        assert!(actual.results.contains_key(&ShortCarClass::DSP));
        assert!(actual.results.contains_key(&ShortCarClass::ES));
        assert!(actual.results.contains_key(&ShortCarClass::FP));
        assert!(actual.results.contains_key(&ShortCarClass::FS));
        assert!(actual.results.contains_key(&ShortCarClass::GS));
        assert!(actual.results.contains_key(&ShortCarClass::HS));
        assert!(actual.results.contains_key(&ShortCarClass::SMF));
        assert!(actual.results.contains_key(&ShortCarClass::SS));
        assert!(actual.results.contains_key(&ShortCarClass::SSC));
        assert!(actual.results.contains_key(&ShortCarClass::SSL));
        assert!(actual.results.contains_key(&ShortCarClass::SSM));
        assert!(actual.results.contains_key(&ShortCarClass::STH));
        assert!(actual.results.contains_key(&ShortCarClass::STR));
        assert!(actual.results.contains_key(&ShortCarClass::STU));
        assert!(actual.results.contains_key(&ShortCarClass::STX));
        assert!(actual.results.contains_key(&ShortCarClass::XA));
        assert!(actual.results.contains_key(&ShortCarClass::XB));
        assert!(actual.results.contains_key(&ShortCarClass::XP));
        assert_eq!(actual.results.len(), 28);

        let a_street = actual.results.get(&ShortCarClass::AS).unwrap();
        assert_eq!(a_street.car_class.short, ShortCarClass::AS);
        assert_eq!(a_street.drivers.len(), 5);
        assert_eq!(
            a_street.get_best_in_class(None),
            LapTime::new(45.269, 0.823, 0, None)
        );
        assert_eq!(a_street.get_best_in_class(Some(TimeSelection::Day2)), dns());
        assert_eq!(a_street.get_best_in_class(Some(TimeSelection::Day2)), dns());

        let robert = a_street.drivers[0].clone();
        assert!(!robert.error);
        assert_eq!(robert.id, "robert fullriede");
        assert_eq!(robert.name, "Robert Fullriede");
        assert_eq!(robert.car_number, 52);
        assert_eq!(robert.car_class.short, ShortCarClass::AS);
        assert_eq!(robert.car_description, "2010 Porsche Cayman");
        assert_eq!(robert.region, "");
        assert!(!robert.rookie);
        assert!(!robert.ladies_championship);
        assert_eq!(robert.position, Some(1));
        assert!(!robert.dsq);
        assert_eq!(robert.pax_multiplier, 0.823);
        assert_eq!(
            robert.day_1_times,
            Some(vec![
                LapTime::new(45.269, 0.823, 0, None),
                LapTime::new(45.519, 0.823, 0, None),
                LapTime::new(45.559, 0.823, 0, None),
                LapTime::new(46.247, 0.823, 0, None),
                LapTime::new(47.069, 0.823, 0, None),
                LapTime::new(48.317, 0.823, 6, None),
            ]),
        );
        assert_eq!(robert.day_2_times, None);
        assert_eq!(robert.combined, LapTime::new(45.269, 0.823, 0, None));

        for (index, driver) in a_street.drivers.iter().enumerate() {
            assert_eq!(driver.position, Some(index + 1));
        }
    }

    #[test]
    fn parse_results_with_no_show_driver() {
        let sample_contents = r#"Position, Class, Class Category, Class Name, Number, First Name,Last Name, Car Year, Car Make, Car Model, Car Color, Member #, Rookie, Ladies, DSQ, Region, Best Run, Pax Index, Pax Time, Runs Day1, Runs Day2, Runs (Time/Cones/Penalty)
"1","SS","Street","Super Street","78","Sean","Greer","2022","Chevrolet","Challenger Cobra 392","urine","432501","0","","0","STL","41.442","0.83","34.397","6","0","42.429","0","","41.862","0","","41.595","0","","41.537","0","","41.445","0","","41.442","0",""
"17","CAMT","Other","Classic American Muscle Traditional","88","Charles","Hammelman","1999","Ford","Mustang SVT Cobra","Black","691686","1","","0","","DNF","0.816","999","0","0""#;

        let actual = parse(sample_contents.to_string(), false);

        assert!(actual.is_err(), "Should fail on empty driver");
        assert_eq!(actual.err().unwrap(), "Encountered an unexpected end of row for a record. One common reason for this is a driver that did not attend the event but remains in Pronto.\n'StringRecord([\"17\", \"CAMT\", \"Other\", \"Classic American Muscle Traditional\", \"88\", \"Charles\", \"Hammelman\", \"1999\", \"Ford\", \"Mustang SVT Cobra\", \"Black\", \"691686\", \"1\", \"\", \"0\", \"\", \"DNF\", \"0.816\", \"999\", \"0\", \"0\"])'");
    }
}
