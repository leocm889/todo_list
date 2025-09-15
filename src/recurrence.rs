use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum Recurrence {
    Daily,
    Weekly,
    Custom(String),
}

impl Display for Recurrence {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Recurrence::Daily => f.write_str("Daily"),
            Recurrence::Weekly => f.write_str("Weekly"),
            Recurrence::Custom(text) => write!(f, "Custom({text})"),
        }
    }
}

impl FromStr for Recurrence {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(Recurrence::Daily),
            "weekly" => Ok(Recurrence::Weekly),
            other => Ok(Recurrence::Custom(other.to_string())),
        }
    }
}
