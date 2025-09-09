use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize, ValueEnum)]
pub enum Recurrence {
    Daily,
    Weekly,
    Custom,
}

impl Display for Recurrence {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let label = match self {
            Recurrence::Daily => "Daily",
            Recurrence::Weekly => "Medium",
            Recurrence::Custom => "Custom",
        };
        write!(f, "{label}")
    }
}
