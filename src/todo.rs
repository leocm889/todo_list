use std::fmt::{Display, Formatter, Result};

use crate::input::AddTodoInput;
use crate::status::Status;
use crate::storage::{load_todos_from_file, save_todos_to_file};
use crate::utils::{read_input, read_optional_input};
use crate::{priority::Priority, recurrence::Recurrence};
use chrono::{DateTime, NaiveDateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub created_at: NaiveDateTime,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
    pub parent_id: Option<Uuid>,
    pub subtasks: Option<Vec<Uuid>>,
    pub recurrence: Option<Recurrence>,
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
        let recurrence_str = match &self.recurrence {
            Some(r) => r.to_string(),
            None => "None".to_string(),
        };
        let due_date_str = match &self.due_date {
            Some(d) => d.to_rfc3339(),
            None => "None".to_string(),
        };
        let tags_str = match &self.tags {
            Some(tags) => tags.join(", "),
            None => "None".to_string(),
        };
        let subtasks_str = match &self.subtasks {
            Some(subs) if !subs.is_empty() => subs
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            _ => "None".to_string(),
        };

        let todo_block = format!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            "ID:".bold(),
            self.id.to_string().cyan(),
            "Title:".bold(),
            self.title.bold(),
            "Description:".bold(),
            self.description.as_deref().unwrap_or("None"),
            "Priority:".bold(),
            priority_color,
            "Status:".bold(),
            status_color,
            "Due Date:".bold(),
            due_date_str,
            "Tags:".bold(),
            tags_str,
            "Parent ID:".bold(),
            self.parent_id
                .map(|id| id.to_string())
                .unwrap_or("None".to_string()),
            "Subtasks:".bold(),
            subtasks_str,
            "Recurrence:".bold(),
            recurrence_str.cyan(),
        );

        if self.is_overdue() {
            write!(
                f,
                "{}\n{}",
                "⚠️ OVERDUE!".red().bold(),
                todo_block.red().bold()
            )
        } else {
            f.write_str(&todo_block)
        }
    }
}

impl Todo {
    pub fn new(input: AddTodoInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: input.title,
            description: input.description,
            priority: input.priority,
            status: input.status,
            created_at: Utc::now().naive_utc(),
            due_date: input.due_date,
            tags: input.tags,
            parent_id: input.parent_id,
            subtasks: input.subtasks,
            recurrence: input.recurrence,
        }
    }

    pub fn is_overdue(&self) -> bool {
        self.due_date.map(|due| Utc::now() > due).unwrap_or(false)
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

    let description = read_optional_input::<String>();

    let priority = read_priority();

    let status = read_status();

    let due_date = read_optional_due_date();

    let tags = read_optional_tags();

    let parent_id = read_optional_uuid("Enter parent task ID (optional):");

    let subtasks = read_optional_uuids("Enter subtasks IDs separated by commas (optional):");

    let recurrence = read_recurrence(None);

    let todo = Todo::new(AddTodoInput {
        title,
        description,
        priority,
        status,
        due_date,
        tags,
        parent_id,
        subtasks,
        recurrence,
    });
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
        println!("{}", "4. Due date".cyan());
        println!("{}", "5. Overdue tasks first".red());
        println!("{}", "6. Back to main menu".red());

        let choice = read_input::<u32>();

        let mut todo_list: Vec<&Todo> = todos.values().collect();

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
            4 => {
                todo_list.sort_by_key(|t| (t.due_date.is_none(), t.due_date));
            }
            5 => {
                todo_list.sort_by_key(|t| !t.is_overdue());
            }
            6 => break,
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
        println!("{}", "5. Due date".cyan());
        println!("{}", "6. Recurrence".magenta());
        println!("{}", "7. Tags".yellow());
        println!("{}", "8. Parent task ID".green());
        println!("{}", "9. Back to main menu".red());

        let choice = read_input::<u32>();

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
            5 => {
                println!(
                    "{}",
                    "Enter due date to search (YYYY-MM-DD HH:MM):".blue().bold()
                );
                if let Some(due_date) = read_optional_due_date() {
                    search_todos(file_path, move |t| {
                        t.due_date.map(|d| d == due_date).unwrap_or(false)
                    });
                }
            }
            6 => {
                println!(
                    "{}",
                    "Enter recurrence to search: (daily, weekly, or custom text)"
                        .blue()
                        .bold()
                );
                if let Some(rec) = read_optional_input::<Recurrence>() {
                    search_todos(file_path, move |t| t.recurrence.as_ref() == Some(&rec));
                }
            }
            7 => {
                println!("{}", "Enter tag to search:".blue().bold());
                if let Some(tag) = read_optional_input::<String>() {
                    search_todos(file_path, move |t| {
                        t.tags.as_ref().is_some_and(|tags| tags.contains(&tag))
                    });
                }
            }
            8 => {
                println!("{}", "Enter parent task ID to search:".blue().bold());
                if let Some(pid) = read_optional_input::<Uuid>() {
                    search_todos(file_path, move |t| t.parent_id == Some(pid));
                }
            }
            9 => break,
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
                        "Enter new description (or press Enter to keep '{:?}'):",
                        todo.description
                    )
                    .blue()
                );
                if let Some(desc) = read_optional_input::<String>() {
                    todo.description = Some(desc);
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

                println!("{}", "Do you want to update due date? (y/n)".blue());
                if read_input::<String>().eq_ignore_ascii_case("y") {
                    todo.due_date = read_optional_due_date();
                }

                println!("{}", "Do you want to update tags? (y/n)".blue());
                if read_input::<String>().eq_ignore_ascii_case("y") {
                    todo.tags = read_optional_tags();
                }

                println!("{}", "Do you want to update parent task? (y/n)".blue());
                if read_input::<String>().eq_ignore_ascii_case("y") {
                    todo.parent_id = read_optional_uuid(&format!(
                        "Enter parent task ID (current: {:?}, press Enter to skip):",
                        todo.parent_id
                    ));
                }

                println!("{}", "Do you want to update recurrence? (y/n)".blue());
                if read_input::<String>().eq_ignore_ascii_case("y") {
                    todo.recurrence = read_recurrence(todo.recurrence.as_ref());
                }

                println!("{}", "Do you want to update subtasks? (y/n)".blue());
                if read_input::<String>().eq_ignore_ascii_case("y") {
                    todo.subtasks = read_optional_uuids(
                        "Enter subtask IDs (comma separated, press enter to Skip):",
                    );
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

        let choice = read_input::<u32>();

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

        let choice = read_input::<u32>();

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

pub fn read_recurrence(current: Option<&Recurrence>) -> Option<Recurrence> {
    loop {
        let current_str = current
            .map(|r| r.to_string())
            .unwrap_or_else(|| "None".to_string());

        let prompt = format!("Choose recurrence (current: {current_str} or press Enter to skip):");
        println!("{}", prompt.blue().bold());
        println!("{}", "1. Daily".green());
        println!("{}", "2. Weekly".yellow());
        println!("{}", "3. Custom".magenta());

        if let Some(choice) = read_optional_input::<u32>() {
            match choice {
                1 => return Some(Recurrence::Daily),
                2 => return Some(Recurrence::Weekly),
                3 => {
                    println!("{}", "Enter custom recurrence description".blue());
                    if let Some(custom) = read_optional_input::<String>() {
                        return Some(Recurrence::Custom(custom));
                    } else {
                        return None;
                    }
                }
                _ => {
                    println!("{}", "❌ Invalid choice, try again.".red().bold());
                    continue;
                }
            }
        } else {
            return current.cloned();
        }
    }
}

pub fn read_optional_due_date() -> Option<DateTime<Utc>> {
    println!(
        "{}",
        "Enter due date (YYY-MM-DD HH:MM, press Enter to skip):"
            .blue()
            .bold()
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();

    if trimmed.is_empty() {
        None
    } else {
        match NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H: %M") {
            Ok(ndt) => Some(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc)),
            Err(_) => {
                println!("{}", "⚠️ Invalid date format, skipping.".yellow());
                None
            }
        }
    }
}

pub fn read_optional_tags() -> Option<Vec<String>> {
    println!(
        "{}",
        "Enter tags separated by commas (optional):".blue().bold()
    );
    if let Some(input) = read_optional_input::<String>() {
        let tags: Vec<String> = input.split(',').map(|s| s.trim().to_string()).collect();
        if tags.is_empty() {
            None
        } else {
            Some(tags)
        }
    } else {
        None
    }
}

pub fn read_optional_uuid(prompt: &str) -> Option<Uuid> {
    println!("{}", prompt.blue().bold());
    read_optional_input::<Uuid>()
}

pub fn read_optional_uuids(prompt: &str) -> Option<Vec<Uuid>> {
    println!("{}", prompt.blue().bold());
    if let Some(input) = read_optional_input::<String>() {
        let ids: Vec<Uuid> = input
            .split(',')
            .filter_map(|s| Uuid::parse_str(s.trim()).ok())
            .collect();
        if ids.is_empty() {
            None
        } else {
            Some(ids)
        }
    } else {
        None
    }
}
