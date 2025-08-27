use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    io,
};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    InProgress,
    Done,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: Uuid,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "ID: {}\nTitle: {}\nDescription: {}\nPriority: {:?}\nStatus: {:?}\n",
            self.id, self.title, self.description, self.priority, self.status
        )
    }
}

impl Task {
    pub fn new(title: String, description: String, priority: Priority, status: Status) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            priority,
            status,
        }
    }
}

pub fn add_task(todos: &mut HashMap<Uuid, Task>) {
    loop {
        println!("Enter title");

        let title = input_trimmed();

        println!("Enter description");

        let description = input_trimmed();

        let priority = read_priority();
    }
}

fn read_priority() -> Priority {
    loop {
        println!("Choose priority");
        println!("1. High");
        println!("2. Medium");
        println!("3. Low");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let mut choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match choice {
            1 => return Priority::High,
            2 => return Priority::Medium,
            3 => return Priority::Low,
            _ => {
                println!("Invalid choice, try again.");
                continue;
            }
        };
    }
}

fn input_trimmed() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
