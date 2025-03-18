use bigdecimal::BigDecimal;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use crate::models::type_aliases::{PaxMultiplier, Time};

#[derive(Copy, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Penalty {
    DNF,
    RRN,
    DSQ,
    DNS,
}

#[derive(Clone)]
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
    #[allow(clippy::wrong_self_convention)]
    pub fn to_string(&self, index: bool, display_cone_count: bool) -> String {
        if self.dnf {
            "DNF".to_string()
        } else if self.rerun {
            "Re-run".to_string()
        } else if self.dsq {
            "DSQ".to_string()
        } else if self.dns {
            "DNS".to_string()
        } else {
            let (self_big, _, self_index) = self.bigs();

            let time_string = format!("{:.3}", if index { self_index.round(3) } else { self_big.round(3) });
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
            rhs
        } else {
            LapTime::new(
                self.raw.clone().unwrap() + rhs.raw.unwrap(),
                self.pax.clone(),
                self.cones + rhs.cones,
                None,
            )
        }
    }

    pub fn compare(&self, rhs: &LapTime) -> i8 {
        self.compare2(rhs, true)
    }

    pub fn compare2(&self, rhs: &LapTime, use_pax: bool) -> i8 {
        match (self.time.clone(), rhs.time.clone()) {
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

    pub fn with_pax(&self) -> Option<Time> {
        self.time.clone().map(|_| self.bigs().2)
    }

    fn bigs(&self) -> (BigDecimal, BigDecimal, BigDecimal) {
        (
            self.time.clone().unwrap(),
            self.pax.clone(),
            (self.time.clone().unwrap().clone() * self.pax.clone().clone()).round(3),
        )
    }
}

impl LapTime {
    pub fn new(raw_time: Time, pax: PaxMultiplier, cones: u8, penalty: Option<Penalty>) -> LapTime {
        match penalty {
            None => LapTime {
                raw: Some(raw_time.clone()),
                time: Some(raw_time + (Time::from(cones)) * 2),
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
        Some(self.cmp(other))
    }
}

impl Ord for LapTime {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.compare(other) {
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialEq for LapTime {
    fn eq(&self, other: &Self) -> bool {
        self.to_string(true, false) == other.to_string(true, false)
    }
}

impl Eq for LapTime {}

pub fn dsq() -> LapTime {
    LapTime::new(Time::from(0), Time::from(1), 0, Some(Penalty::DSQ))
}

pub fn dns() -> LapTime {
    LapTime::new(Time::from(0), Time::from(1), 0, Some(Penalty::DNS))
}

impl Display for LapTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string(false, true).as_str())
    }
}

impl Debug for LapTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use crate::models::lap_time::{dns, dsq, LapTime, Penalty};
    use crate::models::type_aliases::{PaxMultiplier, Time};
    use bigdecimal::Zero;
    use std::str::FromStr;

    #[test]
    fn constructor_should_build_valid_time_without_cones() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.9").unwrap(),
            0,
            None,
        );
        assert_eq!(actual.raw, Some(Time::from_str("12.34").unwrap()));
        assert_eq!(actual.time, Some(Time::from_str("12.34").unwrap()));
        assert_eq!(actual.pax, PaxMultiplier::from_str("0.9").unwrap());
        assert_eq!(actual.cones, 0);
        assert!(!actual.dnf);
        assert!(!actual.rerun);
        assert!(!actual.dsq);
        assert!(!actual.dns);
    }

    #[test]
    fn constructor_should_build_valid_time_with_cones() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.9").unwrap(),
            2,
            None,
        );
        assert_eq!(actual.raw, Some(Time::from_str("12.34").unwrap()));
        assert_eq!(actual.time, Some(Time::from_str("16.34").unwrap()));
        assert_eq!(actual.pax, PaxMultiplier::from_str("0.9").unwrap());
        assert_eq!(actual.cones, 2);
        assert!(!actual.dnf);
        assert!(!actual.rerun);
        assert!(!actual.dsq);
        assert!(!actual.dns);
    }

    #[test]
    fn constructor_should_build_with_dnf() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.9").unwrap(),
            2,
            Some(Penalty::DNF),
        );
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, PaxMultiplier::from_str("0.9").unwrap());
        assert_eq!(actual.cones, 0);
        assert!(actual.dnf);
        assert!(!actual.rerun);
        assert!(!actual.dsq);
        assert!(!actual.dns);
    }

    #[test]
    fn constructor_should_build_with_rerun() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.9").unwrap(),
            2,
            Some(Penalty::RRN),
        );
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, PaxMultiplier::from_str("0.9").unwrap());
        assert_eq!(actual.cones, 0);
        assert!(!actual.dnf);
        assert!(actual.rerun);
        assert!(!actual.dsq);
        assert!(!actual.dns);
    }

    #[test]
    fn constructor_should_build_with_dsq() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.9").unwrap(),
            2,
            Some(Penalty::DSQ),
        );
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, PaxMultiplier::from_str("0.9").unwrap());
        assert_eq!(actual.cones, 0);
        assert!(!actual.dnf);
        assert!(!actual.rerun);
        assert!(actual.dsq);
        assert!(!actual.dns);

        let actual = dsq();
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert!(!actual.dnf);
        assert!(!actual.rerun);
        assert!(actual.dsq);
        assert!(!actual.dns);
    }

    #[test]
    fn constructor_should_build_with_dns() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.9").unwrap(),
            2,
            Some(Penalty::DNS),
        );
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.pax, PaxMultiplier::from_str("0.9").unwrap());
        assert_eq!(actual.cones, 0);
        assert!(!actual.dnf);
        assert!(!actual.rerun);
        assert!(!actual.dsq);
        assert!(actual.dns);

        let actual = dns();
        assert_eq!(actual.raw, None);
        assert_eq!(actual.time, None);
        assert_eq!(actual.cones, 0);
        assert!(!actual.dnf);
        assert!(!actual.rerun);
        assert!(!actual.dsq);
        assert!(actual.dns);
    }

    #[test]
    fn to_string_should_display_with_correct_precision() {
        assert_eq!(
            LapTime::new(Time::from_str("3.1234").unwrap(), PaxMultiplier::zero(), 0, None).to_string(false, true),
            "3.123"
        );
        assert_eq!(
            LapTime::new(Time::from_str("3.1235").unwrap(), 0.into(), 0, None).to_string(false, true),
            "3.124"
        );
        assert_eq!(
            LapTime::new(3.into(), 0.into(), 0, None).to_string(false, true),
            "3.000"
        );
    }

    #[test]
    fn to_string_should_display_valid_time_without_cones() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            0,
            None,
        );
        assert_eq!(actual.to_string(false, true), "12.340".to_string());
        assert_eq!(actual.to_string(true, true), "6.170".to_string());
        assert_eq!(actual.to_string(false, false), "12.340".to_string());
        assert_eq!(actual.to_string(true, false), "6.170".to_string());

        assert_eq!(
            LapTime::new(
                Time::from_str("100.34").unwrap(),
                PaxMultiplier::from_str("0.9").unwrap(),
                0,
                None
            )
            .to_string(false, true),
            "100.340".to_string()
        );
        assert_eq!(
            LapTime::new(
                Time::from_str("0.34").unwrap(),
                PaxMultiplier::from_str("0.9").unwrap(),
                0,
                None
            )
            .to_string(false, true),
            "0.340".to_string()
        );
    }

    #[test]
    fn to_string_should_display_valid_time_with_cones() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            2,
            None,
        );
        assert_eq!(actual.to_string(false, true), "16.340 (2)".to_string());
        assert_eq!(actual.to_string(true, true), "8.170 (2)".to_string());
        assert_eq!(actual.to_string(false, false), "16.340".to_string());
        assert_eq!(actual.to_string(true, false), "8.170".to_string());
    }

    #[test]
    fn to_string_should_display_with_dnf() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            2,
            Some(Penalty::DNF),
        );
        assert_eq!(actual.to_string(false, true), "DNF".to_string());
        assert_eq!(actual.to_string(true, true), "DNF".to_string());
        assert_eq!(actual.to_string(false, false), "DNF".to_string());
        assert_eq!(actual.to_string(true, false), "DNF".to_string());
    }

    #[test]
    fn to_string_should_display_with_rerun() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            2,
            Some(Penalty::RRN),
        );
        assert_eq!(actual.to_string(false, true), "Re-run".to_string());
        assert_eq!(actual.to_string(true, true), "Re-run".to_string());
        assert_eq!(actual.to_string(false, false), "Re-run".to_string());
        assert_eq!(actual.to_string(true, false), "Re-run".to_string());
    }

    #[test]
    fn to_string_should_display_with_dsq() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            2,
            Some(Penalty::DSQ),
        );
        assert_eq!(actual.to_string(false, true), "DSQ".to_string());
        assert_eq!(actual.to_string(true, true), "DSQ".to_string());
        assert_eq!(actual.to_string(false, false), "DSQ".to_string());
        assert_eq!(actual.to_string(true, false), "DSQ".to_string());
    }

    #[test]
    fn to_string_should_display_with_dns() {
        let actual = LapTime::new(
            Time::from_str("12.34").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            2,
            Some(Penalty::DNS),
        );
        assert_eq!(actual.to_string(false, true), "DNS".to_string());
        assert_eq!(actual.to_string(true, true), "DNS".to_string());
        assert_eq!(actual.to_string(false, false), "DNS".to_string());
        assert_eq!(actual.to_string(true, false), "DNS".to_string());
    }

    #[test]
    fn comparator_should_sort_correctly() {
        let mut actual = vec![
            LapTime::new(10.into(), 1.into(), 0, None),
            LapTime::new(6.into(), 1.into(), 1, None),
            LapTime::new(1.into(), 1.into(), 0, Some(Penalty::DNF)),
            LapTime::new(1.into(), 1.into(), 0, Some(Penalty::DNS)),
            LapTime::new(1.into(), 1.into(), 0, Some(Penalty::RRN)),
            LapTime::new(1.into(), 1.into(), 0, Some(Penalty::DSQ)),
            LapTime::new(12.into(), 1.into(), 0, None),
            LapTime::new(12.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
            LapTime::new(7.into(), 1.into(), 0, None),
        ];

        actual.sort();

        assert_eq!(
            actual.first().unwrap().clone(),
            LapTime::new(12.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
            ""
        );
        assert_eq!(
            actual.get(1).unwrap().clone(),
            LapTime::new(7.into(), 1.into(), 0, None)
        );
        assert_eq!(
            actual.get(2).unwrap().clone(),
            LapTime::new(6.into(), 1.into(), 1, None)
        );
        assert_eq!(
            actual.get(3).unwrap().clone(),
            LapTime::new(10.into(), 1.into(), 0, None)
        );
        assert_eq!(
            actual.get(4).unwrap().clone(),
            LapTime::new(12.into(), 1.into(), 0, None)
        );
        assert_eq!(actual.get(5).unwrap().time, None);
        assert_eq!(actual.get(6).unwrap().time, None);
        assert_eq!(actual.get(7).unwrap().time, None);
        assert_eq!(actual.get(8).unwrap().time, None);
    }

    #[test]
    fn add_should_add_two_times_without_cones() {
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "8.000");
    }

    #[test]
    fn add_should_add_two_times_with_cones() {
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "10.000 (1)");

        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 2, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "12.000 (2)");

        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 2, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "14.000 (3)");
    }

    #[test]
    fn add_should_add_two_times_with_penalties() {
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::DNF));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNF");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::DNS));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNS");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::DSQ));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "DSQ");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::RRN));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None);
        assert_eq!(lhs.add(rhs).to_string(false, true), "Re-run");

        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::DNF));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNF");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::DNS));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNS");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::DSQ));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DSQ");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, None);
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::RRN));
        assert_eq!(lhs.add(rhs).to_string(false, true), "Re-run");

        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::DNS));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::DNF));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNS");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::DSQ));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::DNS));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DSQ");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::RRN));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::DSQ));
        assert_eq!(lhs.add(rhs).to_string(false, true), "Re-run");
        let lhs = LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 1, Some(Penalty::DNF));
        let rhs = LapTime::new(5.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, Some(Penalty::RRN));
        assert_eq!(lhs.add(rhs).to_string(false, true), "DNF");
    }
}
