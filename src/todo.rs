use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    fs::read,
    io,
};

use crate::priority::Priority;
use crate::status::Status;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    id: Uuid,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
    created_at: NaiveDateTime,
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
            created_at: Utc::now().naive_utc(),
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

pub fn retrieve_todos_sorted(todos: &HashMap<Uuid, Todo>) {
    loop {
        if todos.is_empty() {
            println!("No todos in your list yet.");
            return;
        }

        println!("Sort todos by: ");
        println!("1. Priority");
        println!("2. Status");
        println!("3. Creation order");
        println!("4. Back to main menu");

        let mut choice = String::new();

        let mut todo_list: Vec<&Todo> = todos.values().collect();

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
            1 => {
                todo_list.sort_by_key(|t| t.priority.clone());
            }
            2 => {
                todo_list.sort_by_key(|t| t.status.clone());
            }
            3 => {
                todo_list.sort_by_key(|t| t.created_at.clone());
            }
            4 => break,
            _ => {
                println!("Invalid choice, displaying in default creation order.");
            }
        }

        println!("--- Todos ---");
        for todo in &todo_list {
            println!("{todo}");
        }
    }
}

pub fn search_todos<F>(todos: &HashMap<Uuid, Todo>, predicate: F)
where
    F: Fn(&Todo) -> bool,
{
    let results: Vec<&Todo> = todos.values().filter(|todo| predicate(todo)).collect();

    if results.is_empty() {
        println!("No todos found matching the criteria.");
    } else {
        println!("Found {} todo(s):", results.len());
        for todo in results {
            println!("{todo}");
        }
    }
}

pub fn search_todo_by_id(todos: &HashMap<Uuid, Todo>, id: Uuid) {
    match todos.get(&id) {
        Some(todo) => println!("Todo found:\n{todo}"),
        None => println!("No item found with ID: {id}"),
    }
}

pub fn search_todo_by_title(todos: &HashMap<Uuid, Todo>, query: &str) {
    let query_lower = query.to_lowercase();
    search_todos(todos, |todo| {
        todo.title.to_lowercase().contains(&query_lower)
    });
}

pub fn search_todo_by_priority(todos: &HashMap<Uuid, Todo>, priority: Priority) {
    search_todos(todos, |todo| todo.priority == priority);
}

pub fn search_todo_by_status(todos: &HashMap<Uuid, Todo>, status: Status) {
    search_todos(todos, |todo| todo.status == status);
}

pub fn search_menu(todos: &HashMap<Uuid, Todo>) {
    loop {
        println!("Search by:");
        println!("1. ID");
        println!("2. Title");
        println!("3. Priority");
        println!("4. Status");
        println!("5. Back to main menu");

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
            1 => {
                println!("Enter the ID to search:");
                let id_input = input_trimmed();
                match Uuid::parse_str(&id_input) {
                    Ok(id) => {
                        search_todo_by_id(todos, id);
                    }
                    Err(_) => println!("Invalid UUID format."),
                }
            }
            2 => {
                println!("Enter the title of the todo to search:");
                let title_query = input_trimmed();
                search_todo_by_title(todos, &title_query);
            }
            3 => {
                let priority = read_priority();
                search_todo_by_priority(todos, priority);
            }
            4 => {
                let status = read_status();
                search_todo_by_status(todos, status);
            }
            5 => break,
            _ => println!("Invalid choice, try again."),
        }
    }
}

pub fn update_todo(todos: &mut HashMap<Uuid, Todo>) {
    println!("Please enter the id of the todo you would like to update:");
    let id_input = input_trimmed();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if let Some(todo) = todos.get_mut(&id) {
                println!("Updating todo: \n{todo}");

                println!("Enter new title (or press Enter to keep '{}'):", todo.title);
                let title = input_trimmed();
                if !title.is_empty() {
                    todo.title = title;
                }

                println!(
                    "Enter new description (or press Enter to keep '{}'):",
                    todo.description
                );

                let description = input_trimmed();
                if !description.is_empty() {
                    todo.description = description;
                }

                println!("Do you want to update priority? (y/n)");
                let choice = input_trimmed();
                if choice.eq_ignore_ascii_case("y") {
                    todo.priority = read_priority();
                }

                println!("Do you want to update status? (y/n)");
                let choice = input_trimmed();
                if choice.eq_ignore_ascii_case("y") {
                    todo.status = read_status();
                }

                println!("Todo updated successfully.");
            } else {
                println!("No todo found with ID: {id}");
            }
        }
        Err(_) => println!("Invalid UUID format."),
    }
}

pub fn delete_todo(todos: &mut HashMap<Uuid, Todo>) {
    println!("Please enter the id of the todo you would like to delete:");
    let id_input = input_trimmed();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if todos.remove(&id).is_some() {
                println!("Todo deleted successfully");
            } else {
                println!("No todo found with id: {id}");
            }
        }
        Err(_) => println!("Invalid UUID format."),
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

pub fn save_todos_to_file(todos: &HashMap<Uuid, Todo>, file_path: &str) {}
