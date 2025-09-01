mod priority;
mod status;
mod todo;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    use super::*;
    use crate::priority::Priority;
    use crate::status::Status;
    use crate::todo::{add_todo, load_todos_from_file, save_todos_to_file, Todo};
    use chrono::Utc;
    use uuid::Uuid;

    fn add_todo_to_map(
        todos: &mut HashMap<Uuid, Todo>,
        title: String,
        description: String,
        priority: Priority,
        status: Status,
    ) -> Uuid {
        let todo = Todo::new(title, description, priority, status);
        let id = todo.id;
        todos.insert(id, todo);
        id
    }

    fn update_todo_in_map(
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
                todo.description = description;
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

    fn delete_todo_by_id(todos: &mut HashMap<Uuid, Todo>, id: Uuid) -> bool {
        todos.remove(&id).is_some()
    }

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
        assert_eq!(todo.description, "New Desc");
        assert_eq!(todo.priority, Priority::High);
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

        assert!(updated);
    }
}
