use crate::storage::load_todos_from_file;
use chrono::Utc;
use notify_rust::Notification;

pub fn send_due_notifications(file_path: &str) {
    let todos = load_todos_from_file(file_path);
    if todos.is_empty() {
        return;
    }

    let now = Utc::now();
    for (_id, todo) in todos.iter() {
        if let Some(due) = todo.due_date {
            if due <= now {
                let title = format!(
                    "{}: {}",
                    if due < now { "Overdue" } else { "Due" },
                    todo.title
                );
                let body = format!(
                    "{}\nPriority: {}\nStatus: {}\nDue: {}",
                    todo.description
                        .as_deref()
                        .unwrap_or("No description provided"),
                    todo.priority,
                    todo.status,
                    due
                );
                let _ = Notification::new().summary(&title).body(&body).show();
            }
        }
    }
}

