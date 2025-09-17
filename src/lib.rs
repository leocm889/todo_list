pub mod input;
pub mod priority;
pub mod recurrence;
pub mod status;
pub mod storage;
pub mod todo;
pub mod utils;
use crate::status::Status;
use crate::todo::Todo;
use crate::{input::AddTodoInput, priority::Priority};
use std::collections::HashMap;
use uuid::Uuid;

pub fn add_todo_to_map(
    todos: &mut HashMap<Uuid, Todo>,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
) -> Uuid {
    let todo = Todo::new(AddTodoInput {
        title,
        description: Some(description),
        priority,
        status,
        due_date: None,
        tags: None,
        recurrence: None,
        parent_id: None,
        subtasks: None,
    });
    let id = todo.id;
    todos.insert(id, todo);
    id
}

pub fn update_todo_in_map(
    todos: &mut HashMap<Uuid, Todo>,
    id: Uuid,
    new_title: Option<String>,
    new_description: Option<String>,
    new_priority: Option<Priority>,
    new_status: Option<Status>,
) -> bool {
    if let Some(todo) = todos.get_mut(&id) {
        if let Some(title) = new_title {
            todo.title = title;
        }
        if let Some(description) = new_description {
            todo.description = Some(description);
        }
        if let Some(priority) = new_priority {
            todo.priority = priority;
        }
        if let Some(status) = new_status {
            todo.status = status;
        }
        true
    } else {
        false
    }
}

pub fn delete_todo_by_id(todos: &mut HashMap<Uuid, Todo>, id: Uuid) -> bool {
    todos.remove(&id).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_todo_inserts_item() {
        let mut todos = HashMap::new();
        let id = add_todo_to_map(
            &mut todos,
            "Learn Rust".into(),
            "Finish the Rust book".into(),
            Priority::High,
            Status::Pending,
        );

        assert!(todos.contains_key(&id));
        assert_eq!(todos[&id].title, "Learn Rust");
    }

    #[test]
    fn update_todo_changes_fields() {
        let mut todos = HashMap::new();

        let id = add_todo_to_map(
            &mut todos,
            "Old Title".into(),
            "Old description".into(),
            Priority::Low,
            Status::Pending,
        );

        let updated = update_todo_in_map(
            &mut todos,
            id,
            Some("New Title".into()),
            Some("New Desc".into()),
            Some(Priority::Medium),
            Some(Status::Done),
        );

        assert!(updated);
        let todo = &todos[&id];
        assert_eq!(todo.title, "New Title");
        assert_eq!(todo.description.as_deref(), Some("New Desc"));
        assert_eq!(todo.priority, Priority::Medium);
        assert_eq!(todo.status, Status::Done);
    }

    #[test]
    fn delete_todo_removes_item() {
        let mut todos = HashMap::new();
        let id = add_todo_to_map(
            &mut todos,
            "Temporary".into(),
            "To be deleted".into(),
            Priority::Medium,
            Status::Pending,
        );

        let deleted = delete_todo_by_id(&mut todos, id);

        assert!(deleted);
        assert!(!todos.contains_key(&id));
    }

    #[test]
    fn update_todo_nonexistent_returns_false() {
        let mut todos = HashMap::new();
        let random_id = Uuid::new_v4();

        let updated = update_todo_in_map(
            &mut todos,
            random_id,
            Some("Doesn't matter".into()),
            None,
            None,
            None,
        );

        assert!(!updated);
    }
}
