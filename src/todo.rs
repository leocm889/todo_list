use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    io,
};

use crate::priority::Priority;
use crate::status::Status;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    id: Uuid,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "ID: {}\nTitle: {}\nDescription: {}\nPriority: {:?}\nStatus: {:?}\n",
            self.id, self.title, self.description, self.priority, self.status
        )
    }
}

impl Todo {
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

pub fn add_todo(todos: &mut HashMap<Uuid, Todo>) {
    println!("Enter title");

    let title = input_trimmed();

    println!("Enter description");

    let description = input_trimmed();

    let priority = read_priority();

    let status = read_status();

    let todo = Todo::new(title, description, priority, status);
    todos.insert(todo.id, todo);
    println!("Todo added successfully.");
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

        let choice: u32 = match choice.trim().parse() {
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

fn read_status() -> Status {
    loop {
        println!("Choose status:");
        println!("1. Pending");
        println!("2. In Progress");
        println!("3. Done");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match choice {
            1 => return Status::Pending,
            2 => return Status::InProgress,
            3 => return Status::Done,
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
