use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use float_cmp;
use float_cmp::F64Margin;
use getset::Getters;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::models::type_aliases::{PaxMultiplier, Time};

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Penalty {
    DNF,
    RRN,
    DSQ,
    DNS,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Getters)]
pub struct LapTime {
    pub raw: Option<Time>,
    pub time: Option<Time>,
    pub cones: u8,
    pub dnf: bool,
    pub rerun: bool,
    pub dsq: bool,
    pub dns: bool,
}

#[wasm_bindgen]
impl LapTime {
    #[wasm_bindgen(constructor)]
    pub fn new(raw_time: Time, cones: u8, penalty: Option<Penalty>) -> LapTime {
        match penalty {
            None => LapTime {
                raw: Some(raw_time),
                time: Some(raw_time + (cones as Time) * 2.),
                cones,
                dnf: false,
                rerun: false,
                dsq: false,
                dns: false,
            },
            Some(Penalty::DNF) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: true,
                rerun: false,
                dsq: false,
                dns: false,
            },
            Some(Penalty::RRN) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: false,
                rerun: true,
                dsq: false,
                dns: false,
            },
            Some(Penalty::DSQ) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: false,
                rerun: false,
                dsq: true,
                dns: false,
            },
            Some(Penalty::DNS) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: false,
                rerun: false,
                dsq: false,
                dns: true,
            },
        }
    }

    pub fn to_string(
        &self,
        pax_multiplier: Option<PaxMultiplier>,
        display_cone_count: Option<bool>,
    ) -> String {
        if self.dnf {
            String::from("DNF")
        } else if self.rerun {
            String::from("Re-run")
        } else if self.dsq {
            String::from("DSQ")
        } else if self.dns {
            String::from("DNS")
        } else {
            let result = format!("{:.3}", self.time.unwrap() * pax_multiplier.unwrap_or(1.));
            if display_cone_count.unwrap_or(true) && self.cones != 0 {
                format!("{} ({})", result, self.cones)
            } else {
                result
            }
        }
    }

    pub fn add(&self, rhs: LapTime) -> LapTime {
        if self.dnf || self.rerun || self.dsq || self.dns {
            self.clone()
        } else if rhs.dnf || rhs.rerun || rhs.dsq || rhs.dns {
            rhs.clone()
        } else {
            LapTime::new(
                self.raw.unwrap() + rhs.raw.unwrap(),
                self.cones + rhs.cones,
                None,
            )
        }
    }

    pub fn compare(self: &LapTime, rhs: &LapTime) -> i8 {
        match self.cmp(rhs) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        }
    }
}

impl PartialOrd<Self> for LapTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time.is_none() && other.time.is_none() {
            Some(Ordering::Equal)
        } else if self.time.is_none() {
            Some(Ordering::Greater)
        } else if other.time.is_none() {
            Some(Ordering::Less)
        } else {
            let self_time = self.time.unwrap();
            let other_time = other.time.unwrap();

            if float_cmp::ApproxEq::approx_eq(self_time, other_time, F64Margin::default()) {
                Some(Ordering::Equal)
            } else {
                self_time.partial_cmp(&other_time)
            }
        }
    }
}

impl Ord for LapTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialEq for LapTime {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
            && self.time == other.time
            && self.cones == other.cones
            && self.dnf == other.dnf
            && self.rerun == other.rerun
            && self.dsq == other.dsq
            && self.dns == other.dns
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for LapTime {}

pub fn dsq() -> LapTime {
    LapTime::new(0., 0, Some(Penalty::DSQ))
}

pub fn dns() -> LapTime {
    LapTime::new(0., 0, Some(Penalty::DNS))
}

impl fmt::Display for LapTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(None, None))
    }
}

#[cfg(test)]
mod test {
    use crate::models::lap_time::{dns, dsq, LapTime, Penalty};

    #[test]
    fn constructor_should_build_valid_time_without_cones() {
        let actual = LapTime::new(12.34, 0, None);
        assert_eq!(actual.raw, Some(12.34));
        assert_eq!(actual.time, Some(12.34));
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_valid_time_with_cones() {
        let actual = LapTime::new(12.34, 2, None);
        assert_eq!(actual.raw, Some(12.34));
        assert_eq!(actual.time, Some(16.34));
        assert_eq!(actual.cones, 2);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_with_dnf() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::DNF));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, true);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_with_rerun() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::RRN));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, true);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_with_dsq() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::DSQ));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, true);
        assert_eq!(actual.dns, false);

        let actual = dsq();
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, true);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_with_dns() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::DNS));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, true);

        let actual = dns();
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, true);
    }

    #[test]
    fn to_string_should_display_valid_time_without_cones() {
        let actual = LapTime::new(12.34, 0, None);
        assert_eq!(actual.to_string(None, None), String::from("12.340"));
        assert_eq!(actual.to_string(Some(0.5), None), String::from("6.170"));
        assert_eq!(actual.to_string(None, Some(false)), String::from("12.340"));
        assert_eq!(actual.to_string(None, Some(true)), String::from("12.340"));
        assert_eq!(
            actual.to_string(Some(0.5), Some(false)),
            String::from("6.170")
        );

        assert_eq!(
            LapTime::new(100.34, 0, None).to_string(None, None),
            String::from("100.340")
        );
        assert_eq!(
            LapTime::new(0.34, 0, None).to_string(None, None),
            String::from("0.340")
        );
    }

    #[test]
    fn to_string_should_display_valid_time_with_cones() {
        let actual = LapTime::new(12.34, 2, None);
        assert_eq!(actual.to_string(None, None), String::from("16.340 (2)"));
        assert_eq!(actual.to_string(Some(0.5), None), String::from("8.170 (2)"));
        assert_eq!(actual.to_string(None, Some(false)), String::from("16.340"));
        assert_eq!(
            actual.to_string(None, Some(true)),
            String::from("16.340 (2)")
        );
        assert_eq!(
            actual.to_string(Some(0.5), Some(false)),
            String::from("8.170")
        );
    }

    #[test]
    fn to_string_should_display_with_dnf() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::DNF));
        assert_eq!(actual.to_string(None, None), String::from("DNF"));
        assert_eq!(actual.to_string(Some(0.5), None), String::from("DNF"));
        assert_eq!(actual.to_string(None, Some(false)), String::from("DNF"));
        assert_eq!(actual.to_string(None, Some(true)), String::from("DNF"));
        assert_eq!(
            actual.to_string(Some(0.5), Some(false)),
            String::from("DNF")
        );
    }

    #[test]
    fn to_string_should_display_with_rerun() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::RRN));
        assert_eq!(actual.to_string(None, None), String::from("Re-run"));
        assert_eq!(actual.to_string(Some(0.5), None), String::from("Re-run"));
        assert_eq!(actual.to_string(None, Some(false)), String::from("Re-run"));
        assert_eq!(actual.to_string(None, Some(true)), String::from("Re-run"));
        assert_eq!(
            actual.to_string(Some(0.5), Some(false)),
            String::from("Re-run")
        );
    }

    #[test]
    fn to_string_should_display_with_dsq() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::DSQ));
        assert_eq!(actual.to_string(None, None), String::from("DSQ"));
        assert_eq!(actual.to_string(Some(0.5), None), String::from("DSQ"));
        assert_eq!(actual.to_string(None, Some(false)), String::from("DSQ"));
        assert_eq!(actual.to_string(None, Some(true)), String::from("DSQ"));
        assert_eq!(
            actual.to_string(Some(0.5), Some(false)),
            String::from("DSQ")
        );
    }

    #[test]
    fn to_string_should_display_with_dns() {
        let actual = LapTime::new(12.34, 2, Some(Penalty::DNS));
        assert_eq!(actual.to_string(None, None), String::from("DNS"));
        assert_eq!(actual.to_string(Some(0.5), None), String::from("DNS"));
        assert_eq!(actual.to_string(None, Some(false)), String::from("DNS"));
        assert_eq!(actual.to_string(None, Some(true)), String::from("DNS"));
        assert_eq!(
            actual.to_string(Some(0.5), Some(false)),
            String::from("DNS")
        );
    }

    #[test]
    fn comparator_should_sort_correctly() {
        let mut actual = Vec::new();
        actual.push(LapTime::new(10., 0, None));
        actual.push(LapTime::new(6., 1, None));
        actual.push(LapTime::new(1., 0, Some(Penalty::DNF)));
        actual.push(LapTime::new(1., 0, Some(Penalty::DNS)));
        actual.push(LapTime::new(1., 0, Some(Penalty::RRN)));
        actual.push(LapTime::new(1., 0, Some(Penalty::DSQ)));
        actual.push(LapTime::new(12., 0, None));
        actual.push(LapTime::new(7., 0, None));

        actual.sort();

        assert_eq!(actual.get(0).unwrap().time, Some(7.));
        assert_eq!(actual.get(1).unwrap().time, Some(8.));
        assert_eq!(actual.get(2).unwrap().time, Some(10.));
        assert_eq!(actual.get(3).unwrap().time, Some(12.));
        assert_eq!(actual.get(4).unwrap().time, None);
        assert_eq!(actual.get(5).unwrap().time, None);
        assert_eq!(actual.get(6).unwrap().time, None);
        assert_eq!(actual.get(7).unwrap().time, None);
    }

    #[test]
    fn add_should_add_two_times_without_cones() {
        let lhs = LapTime::new(3., 0, None);
        let rhs = LapTime::new(5., 0, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "8.000");
    }

    #[test]
    fn add_should_add_two_times_with_cones() {
        let lhs = LapTime::new(3., 1, None);
        let rhs = LapTime::new(5., 0, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "10.000 (1)");

        let lhs = LapTime::new(3., 0, None);
        let rhs = LapTime::new(5., 2, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "12.000 (2)");

        let lhs = LapTime::new(3., 1, None);
        let rhs = LapTime::new(5., 2, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "14.000 (3)");
    }

    #[test]
    fn add_should_add_two_times_with_penalties() {
        let lhs = LapTime::new(3., 1, Some(Penalty::DNF));
        let rhs = LapTime::new(5., 0, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "DNF");
        let lhs = LapTime::new(3., 1, Some(Penalty::DNS));
        let rhs = LapTime::new(5., 0, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "DNS");
        let lhs = LapTime::new(3., 1, Some(Penalty::DSQ));
        let rhs = LapTime::new(5., 0, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "DSQ");
        let lhs = LapTime::new(3., 1, Some(Penalty::RRN));
        let rhs = LapTime::new(5., 0, None);
        assert_eq!(lhs.add(rhs).to_string(None, None), "Re-run");

        let lhs = LapTime::new(3., 1, None);
        let rhs = LapTime::new(5., 0, Some(Penalty::DNF));
        assert_eq!(lhs.add(rhs).to_string(None, None), "DNF");
        let lhs = LapTime::new(3., 1, None);
        let rhs = LapTime::new(5., 0, Some(Penalty::DNS));
        assert_eq!(lhs.add(rhs).to_string(None, None), "DNS");
        let lhs = LapTime::new(3., 1, None);
        let rhs = LapTime::new(5., 0, Some(Penalty::DSQ));
        assert_eq!(lhs.add(rhs).to_string(None, None), "DSQ");
        let lhs = LapTime::new(3., 1, None);
        let rhs = LapTime::new(5., 0, Some(Penalty::RRN));
        assert_eq!(lhs.add(rhs).to_string(None, None), "Re-run");

        let lhs = LapTime::new(3., 1, Some(Penalty::DNS));
        let rhs = LapTime::new(5., 0, Some(Penalty::DNF));
        assert_eq!(lhs.add(rhs).to_string(None, None), "DNS");
        let lhs = LapTime::new(3., 1, Some(Penalty::DSQ));
        let rhs = LapTime::new(5., 0, Some(Penalty::DNS));
        assert_eq!(lhs.add(rhs).to_string(None, None), "DSQ");
        let lhs = LapTime::new(3., 1, Some(Penalty::RRN));
        let rhs = LapTime::new(5., 0, Some(Penalty::DSQ));
        assert_eq!(lhs.add(rhs).to_string(None, None), "Re-run");
        let lhs = LapTime::new(3., 1, Some(Penalty::DNF));
        let rhs = LapTime::new(5., 0, Some(Penalty::RRN));
        assert_eq!(lhs.add(rhs).to_string(None, None), "DNF");
    }
}
