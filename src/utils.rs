use crate::{priority::Priority, status::Status};

pub fn parse_priority(p: &str) -> Priority {
    match p.to_lowercase().as_str() {
        "low" => Priority::Low,
        "medium" => Priority::Medium,
        "high" => Priority::High,
        _ => {
            println!("⚠️ Unknown priority {p}, defaulting to Medium");
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
            println!("⚠️Unknown status {s}, defaulting to Pending");
            Status::Pending
        }
    }
}
