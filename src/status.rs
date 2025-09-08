use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
pub enum Status {
    Pending,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let label = match self {
            Status::Pending => "Pending",
            Status::InProgress => "In Progress",
            Status::Done => "Done",
        };
        write!(f, "{label}")
    }
}
