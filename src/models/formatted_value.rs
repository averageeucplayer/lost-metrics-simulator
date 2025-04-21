const UNITS: &[(&str, u64)] = &[
    ("t", 1_000_000_000_000),
    ("b", 1_000_000_000),
    ("m", 1_000_000),
    ("k", 1_000),
];

#[derive(Debug, Default, Clone, Serialize)]
pub struct FormattedValue {
    pub raw: u64,
    pub value: f64,
    pub unit: &'static str,
    pub formatted: String,
}

impl From<u64> for FormattedValue {
    fn from(n: u64) -> Self {

        for (unit, threshold) in UNITS {
            if n >= *threshold {
                let value = (n as f64) / (*threshold as f64);
                let formatted = format!("{:.1}{}", value, unit);
                return Self {
                    raw: n,
                    value: (n as f64) / (*threshold as f64),
                    unit,
                    formatted
                };
            }
        }

        let formatted = n.to_string();
        Self {
            raw: n,
            value: n as f64,
            unit: "",
            formatted,
        }
    }
}

use std::ops::AddAssign;

use serde::Serialize;

impl AddAssign<u64> for FormattedValue {
    fn add_assign(&mut self, rhs: u64) {
        self.raw += rhs;

        for (unit, threshold) in UNITS {
            if self.raw >= *threshold {
                self.value = (self.raw as f64) / (*threshold as f64);
                self.unit = unit;
                self.formatted = format!("{:.1}{}", self.value, unit);
                return;
            }
        }

        self.value = self.raw as f64;
        self.unit = "";
        self.formatted = self.raw.to_string();
    }
}