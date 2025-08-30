use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    fs::{self, File},
    io::{self, Write},
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

pub fn add_todo(file_path: &str) {
    let mut todos = load_todos_from_file(file_path);

    let id = Uuid::new_v4();

    if todos.contains_key(&id) {
        println!("A todo with this ID already exists. Try again.");
        return;
    }

    println!("Enter title");

    let title = read_input::<String>();

    println!("Enter description");

    let description = read_input::<String>();

    let priority = read_priority();

    let status = read_status();

    let todo = Todo::new(title, description, priority, status);
    todos.insert(todo.id, todo);

    save_todos_to_file(&todos, file_path);

    println!("Todo added successfully.");
}

pub fn retrieve_todos_sorted(file_path: &str) {
    loop {
        let todos = load_todos_from_file(file_path);

        if todos.is_empty() {
            println!("No todos found.");
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

pub fn search_todos<F>(file_path: &str, predicate: F)
where
    F: Fn(&Todo) -> bool,
{
    let todos = load_todos_from_file(file_path);

    if todos.is_empty() {
        println!("No todos found. The file is empty.");
        return;
    }

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

pub fn search_todo_by_id(file_path: &str, id: Uuid) {
    search_todos(file_path, move |todo| todo.id == id);
}

pub fn search_todo_by_title(file_path: &str, query: &str) {
    let query_lower = query.to_lowercase();
    search_todos(file_path, move |todo| {
        todo.title.to_lowercase().contains(&query_lower)
    });
}

pub fn search_todo_by_priority(file_path: &str, priority: Priority) {
    search_todos(file_path, move |todo| todo.priority == priority);
}

pub fn search_todo_by_status(file_path: &str, status: Status) {
    search_todos(file_path, move |todo| todo.status == status);
}

pub fn search_menu(file_path: &str) {
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
                let id_input = read_input::<String>();
                match Uuid::parse_str(&id_input) {
                    Ok(id) => {
                        search_todo_by_id(file_path, id);
                    }
                    Err(_) => println!("Invalid UUID format."),
                }
            }
            2 => {
                println!("Enter the title of the todo to search:");
                let title_query = read_input::<String>();
                search_todo_by_title(file_path, &title_query);
            }
            3 => {
                let priority = read_priority();
                search_todo_by_priority(file_path, priority);
            }
            4 => {
                let status = read_status();
                search_todo_by_status(file_path, status);
            }
            5 => break,
            _ => println!("Invalid choice, try again."),
        }
    }
}

pub fn update_todo(file_path: &str) {
    let mut todos = load_todos_from_file(file_path);

    if todos.is_empty() {
        println!("No todos found. The file is empty.");
        return;
    }

    println!("Please enter the id of the todo you would like to update:");
    let id_input = read_input::<String>();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if let Some(todo) = todos.get_mut(&id) {
                println!("Updating todo: \n{todo}");

                println!("Enter new title (or press Enter to keep '{}'):", todo.title);
                let title = read_optional_input::<String>();
                if let Some(title) = title {
                    todo.title = title;
                }

                println!(
                    "Enter new description (or press Enter to keep '{}'):",
                    todo.description
                );

                let description = read_optional_input::<String>();
                if let Some(description) = description {
                    todo.description = description;
                }

                println!("Do you want to update priority? (y/n)");
                let choice = read_input::<String>();
                if choice.eq_ignore_ascii_case("y") {
                    todo.priority = read_priority();
                }

                println!("Do you want to update status? (y/n)");
                let choice = read_input::<String>();
                if choice.eq_ignore_ascii_case("y") {
                    todo.status = read_status();
                }

                save_todos_to_file(&todos, file_path);
                println!("Todo updated successfully.");
            } else {
                println!("No todo found with ID: {id}");
            }
        }
        Err(_) => println!("Invalid UUID format."),
    }
}

pub fn delete_todo(file_path: &str) {
    let mut todos = load_todos_from_file(file_path);

    if todos.is_empty() {
        println!("No todos found. The file is empty.");
        return;
    }
    println!("Please enter the id of the todo you would like to delete:");
    let id_input = read_input::<String>();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if todos.remove(&id).is_some() {
                save_todos_to_file(&todos, file_path);
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

pub fn read_input<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().ok().expect("Invalid input, try again")
}

pub fn read_optional_input<T: std::str::FromStr>() -> Option<T> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

pub fn save_todos_to_file(todos: &HashMap<Uuid, Todo>, file_path: &str) {
    match File::create(file_path) {
        Ok(mut file) => {
            let json = serde_json::to_string_pretty(todos).unwrap();
            if let Err(error) = file.write_all(json.as_bytes()) {
                eprintln!("Failed to write to file: {error}");
            } else {
                println!("Todos saved successfully to {file_path}");
            }
        }
        Err(error) => eprintln!("Failed to create file: {error}"),
    }
}

pub fn load_todos_from_file(file_path: &str) -> HashMap<Uuid, Todo> {
    match fs::read_to_string(file_path) {
        Ok(data) => match serde_json::from_str::<HashMap<Uuid, Todo>>(&data) {
            Ok(todos) => {
                println!("Todos loaded successfully from {file_path}");
                todos
            }
            Err(error) => {
                eprintln!("Failed to parse JSON: {error}");
                HashMap::new()
            }
        },
        Err(_) => {
            println!("File not found, starting with an empty list of todos.");
            HashMap::new()
        }
    }
}
