use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
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
