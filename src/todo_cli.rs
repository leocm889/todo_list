use crate::priority::Priority;
use crate::status::Status;
use crate::storage::{load_todos_from_file, save_todos_to_file};
use crate::todo::Todo;
use colored::*;
use uuid::Uuid;

pub fn add_todo_cli(
    file_path: &str,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
) {
    let mut todos = load_todos_from_file(file_path);

    let todo = Todo::new(title, description, priority, status);

    if todos.contains_key(&todo.id) {
        println!(
            "{}",
            "❌ A todo with this ID already exists. Try again."
                .red()
                .bold()
        );
        return;
    }

    todos.insert(todo.id, todo);
    save_todos_to_file(&todos, file_path);
    println!("{}", "✅ Todo added successfully".green().bold());
}

pub fn list_todos_cli(file_path: &str, sort_by: &str) {
    let todos = load_todos_from_file(file_path);

    if todos.is_empty() {
        println!("{}", "❌ No todos found.".red().bold());
        return;
    }

    let mut todo_list: Vec<&Todo> = todos.values().collect();

    match sort_by.to_lowercase().as_str() {
        "priority" => todo_list.sort_by_key(|t| t.priority),
        "status" => todo_list.sort_by_key(|t| t.status),
        "created" => todo_list.sort_by_key(|t| t.created_at),
        _ => {
            println!(
                "{}",
                format!("⚠️Unknown sort option '{sort_by}', defaulting to creation order.").red()
            );
            todo_list.sort_by_key(|t| t.created_at);
        }
    }

    println!(
        "{}",
        format!("--- Todos (sorted by {sort_by}) ---")
            .bold()
            .blue()
            .underline()
    );
    for todo in &todo_list {
        let priority_str = todo.priority.to_string();
        let priority_color = match priority_str.as_str() {
            "High" => priority_str.red().bold(),
            "Medium" => priority_str.yellow(),
            "Low" => priority_str.green(),
            _ => priority_str.normal(),
        };

        let status_str = todo.status.to_string();
        let status_color = match status_str.as_str() {
            "Pending" => status_str.red().bold(),
            "In Progress" => status_str.yellow(),
            "Done" => status_str.green(),
            _ => status_str.normal(),
        };

        println!("{:<10} {}", "ID:".bold(), todo.id.to_string().cyan());
        println!("{:<10} {}", "Title:".bold(), todo.title.bold());
        println!("{:<10} {}", "Priority:".bold(), priority_color);
        println!("{:<10} {}", "Status:".bold(), status_color);
        println!("{:<10} {}", "Description:".bold(), todo.description);
        println!("{:<10} {}", "Created:".bold(), todo.created_at);
        println!();
    }
}

pub fn search_todo_cli(
    file_path: &str,
    id: Option<String>,
    title: Option<String>,
    priority: Option<String>,
    status: Option<String>,
) {
    let todos = load_todos_from_file(file_path);
    let mut results: Vec<&Todo> = todos.values().collect();

    if let Some(id_str) = id {
        if let Ok(uuid) = Uuid::parse_str(&id_str) {
            results.retain(|t| t.id == uuid);
        } else {
            println!("{}", format!("⚠️ Invalid UUID format: {id_str}").red());
            return;
        }
    }

    if let Some(title_query) = title {
        let query_lower = title_query.to_lowercase();
        results.retain(|t| t.title.to_lowercase().contains(&query_lower));
    }

    if let Some(priority_str) = priority {
        let p = super::parse_priority(&priority_str);
        results.retain(|t| t.priority == p);
    }

    if let Some(status_str) = status {
        let s = super::parse_status(&status_str);
        results.retain(|t| t.status == s);
    }

    if results.is_empty() {
        println!("{}", "⚠️ No todos found with the given filters.".yellow());
    } else {
        println!(
            "{}",
            format!("Found {} todo(s):", results.len()).bold().blue()
        );
        for todo in results {
            let priority_str = todo.priority.to_string();
            let priority_color = match priority_str.as_str() {
                "High" => priority_str.red().bold(),
                "Medium" => priority_str.yellow(),
                "Low" => priority_str.green(),
                _ => priority_str.normal(),
            };

            let status_str = todo.status.to_string();
            let status_color = match status_str.as_str() {
                "Pending" => status_str.red().bold(),
                "In Progress" => status_str.yellow(),
                "Done" => status_str.green(),
                _ => status_str.normal(),
            };
            println!("{:<10} {}", "ID:".bold(), todo.id.to_string().cyan());
            println!("{:<10} {}", "Title:".bold(), todo.title.bold());
            println!("{:<10} {}", "Priority:".bold(), priority_color);
            println!("{:<10} {}", "Status:".bold(), status_color);
            println!("{:<10} {}", "Description:".bold(), todo.description);
            println!("{:<10} {}", "Created:".bold(), todo.created_at);
            println!();
        }
    }
}

pub fn update_todo_cli(
    file_path: &str,
    id: Uuid,
    new_title: Option<String>,
    new_description: Option<String>,
    new_priority: Option<Priority>,
    new_status: Option<Status>,
) -> bool {
    let mut todos = load_todos_from_file(file_path);

    if let Some(todo) = todos.get_mut(&id) {
        if let Some(title) = new_title {
            todo.title = title
        }
        if let Some(desc) = new_description {
            todo.description = desc
        }
        if let Some(p) = new_priority {
            todo.priority = p;
        }
        if let Some(s) = new_status {
            todo.status = s;
        }
        save_todos_to_file(&todos, file_path);
        true
    } else {
        false
    }
}

pub fn delete_todo_cli(file_path: &str, id: Uuid) -> bool {
    let mut todos = load_todos_from_file(file_path);
    if todos.remove(&id).is_some() {
        save_todos_to_file(&todos, file_path);
        true
    } else {
        false
    }
}
