use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let label = match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };
        write!(f, "{label}")
    }
}
