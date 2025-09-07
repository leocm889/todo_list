use crate::{priority::Priority, status::Status};
use colored::*;

pub fn parse_priority(p: &str) -> Priority {
    match p.to_lowercase().as_str() {
        "low" => Priority::Low,
        "medium" => Priority::Medium,
        "high" => Priority::High,
        _ => {
            println!(
                "{}",
                format!("⚠️ Unknown priority {p}, defaulting to Medium")
                    .yellow()
                    .bold()
            );
            Priority::Medium
        }
    }
}

pub fn parse_status(s: &str) -> Status {
    match s.to_lowercase().as_str() {
        "pending" => Status::Pending,
        "inprogress" | "in_progress" => Status::InProgress,
        "done" => Status::Done,
        _ => {
            println!(
                "{}",
                format!("⚠️Unknown status {s}, defaulting to Pending")
                    .yellow()
                    .bold()
            );
            Status::Pending
        }
    }
}
