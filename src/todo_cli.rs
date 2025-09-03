use crate::priority::Priority;
use crate::status::Status;
use crate::storage::{load_todos_from_file, save_todos_to_file};
use crate::todo::Todo;
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
        println!("A todo with this ID already exists. Try again.");
        return;
    }

    todos.insert(todo.id, todo);
    save_todos_to_file(&todos, file_path);
    println!("✅ Todo added successfully");
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
