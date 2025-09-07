use std::{
    fmt::{Display, Formatter, Result},
    io,
};

use crate::priority::Priority;
use crate::status::Status;
use crate::storage::{load_todos_from_file, save_todos_to_file};
use chrono::{NaiveDateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: Status,
    pub created_at: NaiveDateTime,
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let priority_str = self.priority.to_string();
        let priority_color = match priority_str.as_str() {
            "High" => priority_str.red().bold(),
            "Medium" => priority_str.yellow(),
            "Low" => priority_str.green(),
            _ => priority_str.normal(),
        };
        let status_str = self.status.to_string();
        let status_color = match status_str.as_str() {
            "Pending" => status_str.red().bold(),
            "In Progress" => status_str.yellow(),
            "Done" => status_str.green(),
            _ => status_str.normal(),
        };
        write!(
            f,
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            "ID:".bold(),
            self.id.to_string().cyan(),
            "Title:".bold(),
            self.title.bold(),
            "Description:".bold(),
            self.description,
            "Priority:".bold(),
            priority_color,
            "Status:".bold(),
            status_color,
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
        println!(
            "{}",
            "❌ A todo with this ID already exists. Try again.".red()
        );
        return;
    }

    println!("{}", "Enter title:".blue().bold());

    let title = read_input::<String>();

    println!("{}", "Enter description".blue().bold());

    let description = read_input::<String>();

    let priority = read_priority();

    let status = read_status();

    let todo = Todo::new(title, description, priority, status);
    todos.insert(todo.id, todo);

    save_todos_to_file(&todos, file_path);

    println!("{}", "✅ Todo added successfully.".green().bold());
}

pub fn retrieve_todos_sorted(file_path: &str) {
    loop {
        let todos = load_todos_from_file(file_path);

        if todos.is_empty() {
            println!("{}", "❌ No todos found.".red().bold());
            return;
        }

        println!("{}", "Sort todos by:".blue().bold());
        println!("{}", "1. Priority".yellow());
        println!("{}", "2. Status".green());
        println!("{}", "3. Creation order".magenta());
        println!("{}", "4. Back to main menu".red());

        let mut choice = String::new();

        let mut todo_list: Vec<&Todo> = todos.values().collect();

        io::stdin()
            .read_line(&mut choice)
            .expect("❌ Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️ Please enter a valid number".yellow().bold());
                continue;
            }
        };

        match choice {
            1 => {
                todo_list.sort_by_key(|t| t.priority);
            }
            2 => {
                todo_list.sort_by_key(|t| t.status);
            }
            3 => {
                todo_list.sort_by_key(|t| t.created_at);
            }
            4 => break,
            _ => {
                println!(
                    "{}",
                    "❌ Invalid choice, displaying in default creation order."
                        .red()
                        .bold()
                );
            }
        }

        println!("{}", "--- Todos ---".bold().blue());
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
        println!("{}", "⚠️ No todos found. The file is empty.".yellow());
        return;
    }

    let results: Vec<&Todo> = todos.values().filter(|todo| predicate(todo)).collect();

    if results.is_empty() {
        println!("{}", "⚠️ No todos found matching the criteria.".yellow());
    } else {
        println!(
            "{}",
            format!("Found {} todo(s):", results.len()).green().bold()
        );
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
        println!("{}", "Search by:".blue().bold());
        println!("{}", "1. ID".cyan());
        println!("{}", "2. Title".magenta());
        println!("{}", "3. Priority".yellow());
        println!("{}", "4. Status".green());
        println!("{}", "5. Back to main menu".red());

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("❌ Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️ Please enter a valid number".yellow().bold());
                continue;
            }
        };

        match choice {
            1 => {
                println!("{}", "Enter the ID to search:".blue().bold());
                let id_input = read_input::<String>();
                match Uuid::parse_str(&id_input) {
                    Ok(id) => {
                        search_todo_by_id(file_path, id);
                    }
                    Err(_) => println!("{}", "⚠️ Invalid UUID format.".red()),
                }
            }
            2 => {
                println!("{}", "Enter the title of the todo to search:".blue().bold());
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
            _ => println!("{}", "❌ Invalid choice, try again.".red().bold()),
        }
    }
}

pub fn update_todo(file_path: &str) {
    let mut todos = load_todos_from_file(file_path);

    if todos.is_empty() {
        println!("{}", "⚠️ No todos found. The file is empty.".yellow());
        return;
    }

    println!("Please enter the id of the todo you would like to update:");
    let id_input = read_input::<String>();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if let Some(todo) = todos.get_mut(&id) {
                println!("Updating todo: \n{todo}");

                println!(
                    "{}",
                    format!("Enter new title (or press Enter to keep '{}'):", todo.title).blue()
                );
                let title = read_optional_input::<String>();
                if let Some(title) = title {
                    todo.title = title;
                }

                println!(
                    "{}",
                    format!(
                        "Enter new description (or press Enter to keep '{}'):",
                        todo.description
                    )
                    .blue()
                );

                let description = read_optional_input::<String>();
                if let Some(description) = description {
                    todo.description = description;
                }

                println!("{}", "Do you want to update priority? (y/n)".blue());
                let choice = read_input::<String>();
                if choice.eq_ignore_ascii_case("y") {
                    todo.priority = read_priority();
                }

                println!("{}", "Do you want to update status? (y/n)".blue());
                let choice = read_input::<String>();
                if choice.eq_ignore_ascii_case("y") {
                    todo.status = read_status();
                }

                save_todos_to_file(&todos, file_path);
                println!("{}", "✅ Todo updated successfully.".green().bold());
            } else {
                println!("{}", format!("❌ No todo found with id: {id}").red());
            }
        }
        Err(_) => println!("{}", "⚠️ Invalid UUID format.".red()),
    }
}

pub fn delete_todo(file_path: &str) {
    let mut todos = load_todos_from_file(file_path);

    if todos.is_empty() {
        println!("{}", "⚠️ No todos found. The file is empty.".yellow());
        return;
    }
    println!(
        "{}",
        "Please enter the id of the todo you would like to delete:"
            .blue()
            .bold()
    );
    let id_input = read_input::<String>();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if todos.remove(&id).is_some() {
                save_todos_to_file(&todos, file_path);
                println!("{}", "✅ Todo deleted successfully".green().bold());
            } else {
                println!("{}", format!("❌ No todo found with id: {id}").red());
            }
        }
        Err(_) => println!("{}", "⚠️ Invalid UUID format.".red()),
    }
}

fn read_priority() -> Priority {
    loop {
        println!("{}", "Choose priority".blue().bold());
        println!("{}", "1. High".red().bold());
        println!("{}", "2. Medium".yellow());
        println!("{}", "3. Low".green());

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("❌ Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️ Please enter a valid number".yellow().bold());
                continue;
            }
        };

        match choice {
            1 => return Priority::High,
            2 => return Priority::Medium,
            3 => return Priority::Low,
            _ => {
                println!("{}", "❌ Invalid choice, try again.".red().bold());
                continue;
            }
        };
    }
}

fn read_status() -> Status {
    loop {
        println!("{}", "Choose status:".blue().bold());
        println!("{}", "1. Pending".red().bold());
        println!("{}", "2. In Progress".yellow());
        println!("{}", "3. Done".green());

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("❌ Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️ Please enter a valid number".yellow().bold());
                continue;
            }
        };

        match choice {
            1 => return Status::Pending,
            2 => return Status::InProgress,
            3 => return Status::Done,
            _ => {
                println!("{}", "❌ Invalid choice, try again.".red().bold());
                continue;
            }
        };
    }
}

pub fn read_input<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .parse()
        .ok()
        .expect(&"Invalid input, try again".red().to_string())
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
