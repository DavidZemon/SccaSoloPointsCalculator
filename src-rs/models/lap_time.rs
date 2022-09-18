use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use bigdecimal::{BigDecimal, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::models::type_aliases::{PaxMultiplier, Time};

#[derive(Copy, Clone, Debug)]
pub enum Penalty {
    DNF,
    RRN,
    DSQ,
    DNS,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct LapTime {
    pub raw: Option<Time>,
    pub time: Option<Time>,
    pub pax: PaxMultiplier,
    pub cones: u8,
    pub dnf: bool,
    pub rerun: bool,
    pub dsq: bool,
    pub dns: bool,
}

impl LapTime {
    pub fn to_string(&self, index: bool, display_cone_count: bool) -> String {
        if self.dnf {
            String::from("DNF")
        } else if self.rerun {
            String::from("Re-run")
        } else if self.dsq {
            String::from("DSQ")
        } else if self.dns {
            String::from("DNS")
        } else {
            let (self_big, _, self_index) = self.bigs();

            let time_string = format!(
                "{:.3}",
                if index {
                    self_index.round(3)
                } else {
                    self_big.round(3)
                }
            );
            if display_cone_count && self.cones != 0 {
                format!("{} ({})", time_string, self.cones)
            } else {
                time_string
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
                // Make sure the addition is only performed on integers
                (self.raw.unwrap() * 1000. + rhs.raw.unwrap() * 1000.) / 1000.,
                self.pax,
                self.cones + rhs.cones,
                None,
            )
        }
    }

    pub fn compare(&self, rhs: &LapTime) -> i8 {
        self.compare2(rhs, true)
    }

    pub fn compare2(&self, rhs: &LapTime, use_pax: bool) -> i8 {
        match (self.time, rhs.time) {
            (Some(_), Some(_)) => {
                let (self_raw, _, self_index) = self.bigs();
                let (rhs_raw, _, rhs_index) = rhs.bigs();

                if use_pax {
                    self_index.cmp(&rhs_index) as i8
                } else {
                    self_raw.cmp(&rhs_raw) as i8
                }
            }
            (Some(_), None) => -1,
            (None, Some(_)) => 1,
            _ => 0,
        }
    }

    pub fn with_pax(&self) -> Time {
        match self.time {
            Some(_) => self.bigs().2.round(3).to_f64().unwrap(),
            None => Time::INFINITY,
        }
    }

    fn bigs(&self) -> (BigDecimal, BigDecimal, BigDecimal) {
        let self_big = BigDecimal::from_str(format!("{:.3}", self.time.unwrap()).as_str()).unwrap();
        let self_pax = BigDecimal::from_str(format!("{:.3}", self.pax).as_str()).unwrap();
        let self_index = self_big.clone() * self_pax.clone();

        (self_big.round(3), self_pax.round(3), self_index.round(3))
    }
}

impl LapTime {
    pub fn new(raw_time: Time, pax: PaxMultiplier, cones: u8, penalty: Option<Penalty>) -> LapTime {
        match penalty {
            None => LapTime {
                raw: Some(raw_time),
                time: Some(raw_time + (cones as Time) * 2.),
                pax,
                cones,
                dnf: false,
                rerun: false,
                dsq: false,
                dns: false,
            },
            Some(Penalty::DNF) => LapTime {
                raw: None,
                time: None,
                pax,
                cones: 0,
                dnf: true,
                rerun: false,
                dsq: false,
                dns: false,
            },
            Some(Penalty::RRN) => LapTime {
                raw: None,
                time: None,
                pax,
                cones: 0,
                dnf: false,
                rerun: true,
                dsq: false,
                dns: false,
            },
            Some(Penalty::DSQ) => LapTime {
                raw: None,
                time: None,
                pax,
                cones: 0,
                dnf: false,
                rerun: false,
                dsq: true,
                dns: false,
            },
            Some(Penalty::DNS) => LapTime {
                raw: None,
                time: None,
                pax,
                cones: 0,
                dnf: false,
                rerun: false,
                dsq: false,
                dns: true,
            },
        }
    }

    pub fn partial_cmp2(&self, other: &LapTime, use_pax: bool) -> Option<Ordering> {
        Some(match self.compare2(other, use_pax) {
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => Ordering::Equal,
        })
    }

    pub fn cmp2(&self, other: &LapTime, use_pax: bool) -> Ordering {
        self.partial_cmp2(other, use_pax).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd<Self> for LapTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.compare(other) {
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => Ordering::Equal,
        })
    }
}

impl Ord for LapTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialEq for LapTime {
    fn eq(&self, other: &Self) -> bool {
        self.to_string(true, false) == other.to_string(true, false)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for LapTime {}

pub fn dsq() -> LapTime {
    LapTime::new(0., 1., 0, Some(Penalty::DSQ))
}

pub fn dns() -> LapTime {
    LapTime::new(0., 1., 0, Some(Penalty::DNS))
}

impl Display for LapTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string(false, true).as_str())
    }
}

#[cfg(test)]
mod test {
    use crate::models::lap_time::{dns, dsq, LapTime, Penalty};

    #[test]
    fn constructor_should_build_valid_time_without_cones() {
        let actual = LapTime::new(12.34, 0.9, 0, None);
        assert_eq!(actual.raw, Some(12.34));
        assert_eq!(actual.time, Some(12.34));
        assert_eq!(actual.pax, 0.9);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_valid_time_with_cones() {
        let actual = LapTime::new(12.34, 0.9, 2, None);
        assert_eq!(actual.raw, Some(12.34));
        assert_eq!(actual.time, Some(16.34));
        assert_eq!(actual.pax, 0.9);
        assert_eq!(actual.cones, 2);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_with_dnf() {
        let actual = LapTime::new(12.34, 0.9, 2, Some(Penalty::DNF));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, 0.9);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, true);
        assert_eq!(actual.rerun, false);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_with_rerun() {
        let actual = LapTime::new(12.34, 0.9, 2, Some(Penalty::RRN));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, 0.9);
        assert_eq!(actual.cones, 0);
        assert_eq!(actual.dnf, false);
        assert_eq!(actual.rerun, true);
        assert_eq!(actual.dsq, false);
        assert_eq!(actual.dns, false);
    }

    #[test]
    fn constructor_should_build_with_dsq() {
        let actual = LapTime::new(12.34, 0.9, 2, Some(Penalty::DSQ));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, 0.9);
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
        let actual = LapTime::new(12.34, 0.9, 2, Some(Penalty::DNS));
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, 0.9);
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
        let actual = LapTime::new(12.34, 0.5, 0, None);
        assert_eq!(actual.to_string(false, true), String::from("12.340"));
        assert_eq!(actual.to_string(true, true), String::from("6.170"));
        assert_eq!(actual.to_string(false, false), String::from("12.340"));
        assert_eq!(actual.to_string(true, false), String::from("6.170"));

        assert_eq!(
            LapTime::new(100.34, 0.9, 0, None).to_string(false, true),
            String::from("100.340")
        );
        assert_eq!(
            LapTime::new(0.34, 0.9, 0, None).to_string(false, true),
            String::from("0.340")
        );
    }

    #[test]
    fn to_string_should_display_valid_time_with_cones() {
        let actual = LapTime::new(12.34, 0.5, 2, None);
        assert_eq!(actual.to_string(false, true), String::from("16.340 (2)"));
        assert_eq!(actual.to_string(true, true), String::from("8.170 (2)"));
        assert_eq!(actual.to_string(false, false), String::from("16.340"));
        assert_eq!(actual.to_string(true, false), String::from("8.170"));
    }

    #[test]
    fn to_string_should_display_with_dnf() {
        let actual = LapTime::new(12.34, 0.5, 2, Some(Penalty::DNF));
        assert_eq!(actual.to_string(false, true), String::from("DNF"));
        assert_eq!(actual.to_string(true, true), String::from("DNF"));
        assert_eq!(actual.to_string(false, false), String::from("DNF"));
        assert_eq!(actual.to_string(true, false), String::from("DNF"));
    }

    #[test]
    fn to_string_should_display_with_rerun() {
        let actual = LapTime::new(12.34, 0.5, 2, Some(Penalty::RRN));
        assert_eq!(actual.to_string(false, true), String::from("Re-run"));
        assert_eq!(actual.to_string(true, true), String::from("Re-run"));
        assert_eq!(actual.to_string(false, false), String::from("Re-run"));
        assert_eq!(actual.to_string(true, false), String::from("Re-run"));
    }

    #[test]
    fn to_string_should_display_with_dsq() {
        let actual = LapTime::new(12.34, 0.9, 2, Some(Penalty::DSQ));
        assert_eq!(actual.to_string(false, true), String::from("DSQ"));
        assert_eq!(actual.to_string(true, true), String::from("DSQ"));
        assert_eq!(actual.to_string(false, false), String::from("DSQ"));
        assert_eq!(actual.to_string(true, false), String::from("DSQ"));
    }

    #[test]
    fn to_string_should_display_with_dns() {
        let actual = LapTime::new(12.34, 0.5, 2, Some(Penalty::DNS));
        assert_eq!(actual.to_string(false, true), String::from("DNS"));
        assert_eq!(actual.to_string(true, true), String::from("DNS"));
        assert_eq!(actual.to_string(false, false), String::from("DNS"));
        assert_eq!(actual.to_string(true, false), String::from("DNS"));
    }

    #[test]
    fn comparator_should_sort_correctly() {
        let mut actual = Vec::new();
        actual.push(LapTime::new(10., 1., 0, None));
        actual.push(LapTime::new(6., 1., 1, None));
        actual.push(LapTime::new(1., 1., 0, Some(Penalty::DNF)));
        actual.push(LapTime::new(1., 1., 0, Some(Penalty::DNS)));
        actual.push(LapTime::new(1., 1., 0, Some(Penalty::RRN)));
        actual.push(LapTime::new(1., 1., 0, Some(Penalty::DSQ)));
        actual.push(LapTime::new(12., 1., 0, None));
        actual.push(LapTime::new(12., 0.5, 0, None));
        actual.push(LapTime::new(7., 1., 0, None));

        actual.sort();

        println!("{}", actual.get(0).unwrap());
        println!("{}", actual.get(1).unwrap());
        println!("{}", actual.get(2).unwrap());
        println!("{}", actual.get(3).unwrap());
        println!("{}", actual.get(4).unwrap());
        println!("{}", actual.get(5).unwrap());
        println!("{}", actual.get(6).unwrap());
        println!("{}", actual.get(7).unwrap());
        println!("{}", actual.get(8).unwrap());

        assert_eq!(
            actual.get(0).unwrap().clone(),
            LapTime::new(12., 0.5, 0, None),
            ""
        );
        assert_eq!(
            actual.get(1).unwrap().clone(),
            LapTime::new(7., 1., 0, None)
        );
        assert_eq!(
            actual.get(2).unwrap().clone(),
            LapTime::new(6., 1., 1, None)
        );
        assert_eq!(
            actual.get(3).unwrap().clone(),
            LapTime::new(10., 1., 0, None)
        );
        assert_eq!(
            actual.get(4).unwrap().clone(),
            LapTime::new(12., 1., 0, None)
        );
        assert_eq!(actual.get(5).unwrap().time, None);
        assert_eq!(actual.get(6).unwrap().time, None);
        assert_eq!(actual.get(7).unwrap().time, None);
        assert_eq!(actual.get(8).unwrap().time, None);
    }

    #[test]
    fn add_should_add_two_times_without_cones() {
        let lhs = LapTime::new(3., 0.9, 0, None);
        let rhs = LapTime::new(5., 0.9, 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "8.000");
    }

    #[test]
    fn add_should_add_two_times_with_cones() {
        let lhs = LapTime::new(3., 0.9, 1, None);
        let rhs = LapTime::new(5., 0.9, 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "10.000 (1)");

        let lhs = LapTime::new(3., 0.9, 0, None);
        let rhs = LapTime::new(5., 0.9, 2, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "12.000 (2)");

        let lhs = LapTime::new(3., 0.9, 1, None);
        let rhs = LapTime::new(5., 0.9, 2, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "14.000 (3)");
    }

    #[test]
    fn add_should_add_two_times_with_penalties() {
        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::DNF));
        let rhs = LapTime::new(5., 0.9, 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNF");
        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::DNS));
        let rhs = LapTime::new(5., 0.9, 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNS");
        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::DSQ));
        let rhs = LapTime::new(5., 0.9, 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "DSQ");
        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::RRN));
        let rhs = LapTime::new(5., 0.9, 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "Re-run");

        let lhs = LapTime::new(3., 0.9, 1, None);
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::DNF));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNF");
        let lhs = LapTime::new(3., 0.9, 1, None);
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::DNS));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNS");
        let lhs = LapTime::new(3., 0.9, 1, None);
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::DSQ));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DSQ");
        let lhs = LapTime::new(3., 0.9, 1, None);
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::RRN));
        assert_eq!(lhs.add(rhs).to_string(false, true), "Re-run");

        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::DNS));
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::DNF));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNS");
        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::DSQ));
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::DNS));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DSQ");
        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::RRN));
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::DSQ));
        assert_eq!(lhs.add(rhs).to_string(false, true), "Re-run");
        let lhs = LapTime::new(3., 0.9, 1, Some(Penalty::DNF));
        let rhs = LapTime::new(5., 0.9, 0, Some(Penalty::RRN));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNF");
    }
}
