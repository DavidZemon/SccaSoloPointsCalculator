use crate::models::class_results::ClassResults;
use crate::models::driver::Driver;
use crate::models::driver_from_pronto::DriverFromPronto;
use crate::models::event_results::EventResults;
use crate::models::lap_time::{LapTime, Penalty};
use crate::models::msr_driver::MsrDriver;
use crate::models::type_aliases::{PaxMultiplier, Time};
use bigdecimal::ParseBigDecimalError;
use csv::{StringRecord, Trim};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn parse(msr_export: String, pronto_export: String) -> Result<EventResults, String> {
    let msr_data = parse_msr(msr_export)?;

    let mut pronto_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(pronto_export.as_bytes());

    let mut pronto_string_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(pronto_export.as_bytes());
    let pronto_final_column_index = {
        let headers = pronto_string_reader.headers().map_err(|e| e.to_string())?;
        let header_vec: Vec<&str> = headers.iter().collect();
        header_vec
            .iter()
            .enumerate()
            .find(|(_index, header)| **header == "Runs Day2")
            .map(|(index, _header)| index)
            .ok_or("Unable to find header column `Runs Day2`".to_string())?
    };

    let mut results = HashMap::new();

    let pronto_records = pronto_reader.deserialize().zip(pronto_string_reader.records());
    for (deserialized, string_rec) in pronto_records {
        let (driver, string_rec) = validate_row(deserialized, string_rec)?;

        let driver = extract_lap_times(driver, string_rec, pronto_final_column_index + 1)?;
        let msr_driver = msr_data
            .get(&driver.id())
            .ok_or(format!("Failed to match Pronto driver {} in MSR data", driver.id()))?;
        let driver = Driver::from((driver, msr_driver));

        results
            .entry(driver.car_class.short)
            .or_insert_with(|| ClassResults::new(driver.car_class.short));

        if let Some(class_results) = results.get_mut(&driver.car_class.short) {
            class_results.add_driver(driver)
        }
    }

    Ok(EventResults { results })
}

fn parse_msr(msr_export: String) -> Result<HashMap<String, MsrDriver>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(msr_export.as_bytes());

    let mut string_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(msr_export.as_bytes());

    let records = reader.deserialize().zip(string_reader.records());

    let mut results = HashMap::new();
    for (record, string_rec) in records {
        let record: Option<MsrDriver> =
            record.map_err(|e| format!("Failed to parse MSR driver due to {e:?}:\n\t{string_rec:?}"))?;
        if let Some(driver) = record {
            results.insert(driver.id(), driver);
        }
    }
    Ok(results)
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
                            "Encountered an unexpected end of row for a record. One common reason for this is a driver that did not attend the event but remains in Pronto.\n'{string_record:?}'",
                        )
                    ),
                    _ => Err(format!("Failed to deserialize row {string_record:?} due to {:?}", e.to_string())),
                }
            }
            _ => Err(format!("Failed to deserialize row {string_record:?} due to {:?}", e.to_string())),
        },
    }
}

fn extract_lap_times(
    mut driver: DriverFromPronto,
    string_record: StringRecord,
    first_time_column: usize,
) -> Result<DriverFromPronto, String> {
    let strings_vec: Vec<&str> = string_record.iter().collect();
    let pax_multiplier = PaxMultiplier::from_str(&driver.pax_multiplier).unwrap();

    let extra_fields = &strings_vec[first_time_column..];
    let extra_field_groups = extra_fields.chunks(3);

    let driver_name = format!(
        "{} {}",
        driver.first_name.as_ref().unwrap_or(&"".to_string()),
        driver.last_name.as_ref().unwrap_or(&"".to_string())
    );

    for run in extra_field_groups {
        if run.len() != 3 {
            #[cfg(not(test))]
            crate::console_log!("Driver {driver_name} has run cell count not divisible by 3");
        } else {
            match build_lap_time(run, &pax_multiplier) {
                Ok(run) => {
                    driver.runs.push(run);
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to extract lap time for {driver_name} ({run:?}) due to {e}"
                    ))
                }
            }
        }
    }

    Ok(driver)
}

fn build_lap_time(next_fields: &[&str], pax: &PaxMultiplier) -> Result<LapTime, String> {
    Ok(LapTime::new(
        Time::from_str(next_fields[0]).map_err(|e: ParseBigDecimalError| e.to_string())?,
        pax.clone(),
        next_fields[1].parse().map_err(|e: ParseIntError| e.to_string())?,
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
    use crate::enums::short_car_class::ShortCarClass;
    use crate::models::lap_time::{LapTime, Penalty};
    use crate::models::type_aliases::{PaxMultiplier, Time};
    use crate::services::csv::parser::event_results_parser::parse;
    use bigdecimal::Zero;
    use std::fs;
    use std::str::FromStr;

    // language=CSV
    const MSR_DATA: &str = r###""Last Name","First Name","Member #","Class + Modifier/PAX","No.","Vehicle Year/Make/Model/Color","Region of Record Abbreviation","Medical condition? (Optional)","Is a pro","Novice","Ladies"
Fullriede,Robert,1,AS,1,Vette,STL,,0,0,0
Osborn,Jeffrey,2,AS,2,Foobar,STL,,0,0,0
Sweetwood,Adrian,4,AS,4,Vette,STL,,0,0,0
Buffa,Adam,3,BS,3,Supra,,,0,0,0
Greer,Sean,5,SS,5,Bug,,,0,0,0"###;

    #[test]
    fn parse_2022_e1_event_results() {
        let sample_contents = fs::read_to_string("./SampleData/2022/2022_Event1-DavidExport.csv").unwrap();
        let actual = parse(MSR_DATA.to_string(), sample_contents).unwrap();

        assert_eq!(actual.results.len(), 2);
        assert!(actual.results.contains_key(&ShortCarClass::AS));
        assert!(actual.results.contains_key(&ShortCarClass::BS));

        let a_street = actual.results.get(&ShortCarClass::AS).unwrap();
        assert_eq!(a_street.car_class.short, ShortCarClass::AS);
        assert_eq!(a_street.drivers.len(), 2);
        assert_eq!(
            a_street.get_best_in_class(),
            LapTime::new(
                Time::from_str("52.288").unwrap(),
                PaxMultiplier::from_str("0.821").unwrap(),
                0,
                None
            )
        );
        let robert = a_street.drivers[0].clone();
        assert!(!robert.error);
        assert_eq!(robert.id, "robert fullriede");
        assert_eq!(robert.name, "Robert Fullriede");
        assert_eq!(robert.car_number, 52);
        assert_eq!(robert.car_class.short, ShortCarClass::AS);
        assert_eq!(robert.car_description, "2010 Porsche Cayman");
        assert_eq!(robert.region, "STL");
        assert!(!robert.rookie);
        assert!(!robert.ladies_championship);
        assert_eq!(robert.position, Some(1));
        assert!(!robert.dsq);
        assert_eq!(robert.pax_multiplier, PaxMultiplier::from_str("0.821").unwrap());
        assert_eq!(
            robert.times,
            vec![
                LapTime::new(
                    Time::from_str("53.351").unwrap(),
                    PaxMultiplier::from_str("0.821").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::zero(),
                    PaxMultiplier::from_str("0.821").unwrap(),
                    0,
                    Some(Penalty::DNF)
                ),
                LapTime::new(
                    Time::from_str("52.288").unwrap(),
                    PaxMultiplier::from_str("0.821").unwrap(),
                    0,
                    None
                ),
            ]
        );

        for (index, driver) in a_street.drivers.iter().enumerate() {
            assert_eq!(driver.position, Some(index + 1));
        }
    }

    #[test]
    fn parse_2023_e3_event_results() {
        let sample_contents = fs::read_to_string("./SampleData/2023/2023_Event3-DavidExport.csv").unwrap();

        let actual = parse(MSR_DATA.to_string(), sample_contents).unwrap();
        assert!(actual.results.contains_key(&ShortCarClass::AS));
        assert_eq!(actual.results.len(), 1);

        let a_street = actual.results.get(&ShortCarClass::AS).unwrap();
        assert_eq!(a_street.car_class.short, ShortCarClass::AS);
        assert_eq!(a_street.drivers.len(), 2);
        assert_eq!(
            a_street.get_best_in_class(),
            LapTime::new(
                Time::from_str("45.269").unwrap(),
                PaxMultiplier::from_str("0.823").unwrap(),
                0,
                None
            )
        );

        let robert = a_street.drivers[0].clone();
        assert!(!robert.error);
        assert_eq!(robert.id, "robert fullriede");
        assert_eq!(robert.name, "Robert Fullriede");
        assert_eq!(robert.car_number, 52);
        assert_eq!(robert.car_class.short, ShortCarClass::AS);
        assert_eq!(robert.car_description, "2010 Porsche Cayman");
        assert_eq!(robert.region, "STL");
        assert!(!robert.rookie);
        assert!(!robert.ladies_championship);
        assert_eq!(robert.position, Some(1));
        assert!(!robert.dsq);
        assert_eq!(robert.pax_multiplier, PaxMultiplier::from_str("0.823").unwrap());
        assert_eq!(
            robert.times,
            vec![
                LapTime::new(
                    Time::from_str("48.317").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    6,
                    None
                ),
                LapTime::new(
                    Time::from_str("47.069").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("46.247").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("45.519").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("45.269").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("45.559").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
            ],
        );

        for (index, driver) in a_street.drivers.iter().enumerate() {
            assert_eq!(driver.position, Some(index + 1));
        }
    }

    #[test]
    fn parse_results_with_no_show_driver() {
        // language=csv
        let sample_contents = r#"Position, Class, Class Category, Class Name, Number, First Name,Last Name, Car Year, Car Make, Car Model, Car Color, Member #, Rookie, Ladies, DSQ, Region, Best Run, Pax Index, Pax Time, Runs Day1, Runs Day2, Runs (Time/Cones/Penalty)
"1","SS","Street","Super Street","78","Sean","Greer","2022","Chevrolet","Challenger Cobra 392","urine","432501","0","","0","STL","41.442","0.83","34.397","6","0","42.429","0","","41.862","0","","41.595","0","","41.537","0","","41.445","0","","41.442","0",""
"17","CAMT","Other","Classic American Muscle Traditional","88","Charles","Hammelman","1999","Ford","Mustang SVT Cobra","Black","691686","1","","0","","DNF","0.816","999","0","0""#;

        let actual = parse(MSR_DATA.to_string(), sample_contents.to_string());

        assert!(actual.is_err(), "Should fail on empty driver");
        assert_eq!(actual.err().unwrap(), "Encountered an unexpected end of row for a record. One common reason for this is a driver that did not attend the event but remains in Pronto.\n'StringRecord([\"17\", \"CAMT\", \"Other\", \"Classic American Muscle Traditional\", \"88\", \"Charles\", \"Hammelman\", \"1999\", \"Ford\", \"Mustang SVT Cobra\", \"Black\", \"691686\", \"1\", \"\", \"0\", \"\", \"DNF\", \"0.816\", \"999\", \"0\", \"0\"])'");
    }
}
