use std::collections::HashMap;
use todoscope::priority::Priority;
use todoscope::status::Status;
use todoscope::{add_todo_to_map, delete_todo_by_id, update_todo_in_map};
use uuid::Uuid;

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
        "Old Desc".into(),
        Priority::Low,
        Status::Pending,
    );

    let updated = update_todo_in_map(
        &mut todos,
        id,
        Some("New Title".into()),
        Some("New Desc".into()),
        Some(Priority::High),
        Some(Status::Done),
    );

    assert!(updated);
    let todo = &todos[&id];
    assert_eq!(todo.title, "New Title");
    assert_eq!(todo.description.as_deref(), Some("New Desc"));
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

    assert!(!updated);
}
