use clap::ValueEnum;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortBy {
    Priority,
    Status,
    Created,
    DueDate,
    Overdue,
}

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SortBy::Priority => "Priority",
            SortBy::Status => "Status",
            SortBy::Created => "Created Date",
            SortBy::DueDate => "Due Date",
            SortBy::Overdue => "Overdue",
        };
        write!(f, "{label}")
    }
}
