use std::collections::HashMap;

use wasm_bindgen::JsValue;

use crate::enums::driver_group::DriverGroup;
use crate::enums::short_car_class::ShortCarClass;
use crate::models::class_results::ClassResults;
use crate::models::driver::Driver;

#[derive(Clone, Debug)]
pub struct EventResults {
    pub results: HashMap<ShortCarClass, ClassResults>,
}

impl EventResults {
    /// Driver descriptors (string consisting of name + number + class) of any driver that we
    /// found to be in an error state during import
    pub fn js_drivers_in_error(&self) -> Vec<JsValue> {
        self.results
            .values()
            .map(|class_results| class_results.drivers.iter().filter(|d| d.error))
            .flatten()
            .map(|driver| {
                JsValue::from_str(
                    format!(
                        "{} ({} {})",
                        driver.name,
                        driver.car_number,
                        driver.car_class.short.name()
                    )
                    .as_str(),
                )
            })
            .collect()
    }

    /// Get a sorted list of drivers
    pub fn get_drivers(&self, filter: Option<DriverGroup>) -> Vec<&Driver> {
        let filter = filter.unwrap_or(DriverGroup::PAX);
        let mut drivers = self
            .results
            .values()
            .map(|r| {
                r.drivers.iter().filter(|d| match filter {
                    DriverGroup::Ladies => d.ladies_championship,
                    DriverGroup::Novice => d.rookie,
                    _ => true,
                })
            })
            .flatten()
            .collect::<Vec<&Driver>>();

        drivers.sort_by(|lhs, rhs| {
            lhs.best_lap(None)
                .cmp2(&rhs.best_lap(None), filter != DriverGroup::Raw)
        });
        drivers
    }
}
