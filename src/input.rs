use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{priority::Priority, recurrence::Recurrence, status::Status};

#[derive(Debug, Clone)]
pub struct AddTodoInput {
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
    pub recurrence: Option<Recurrence>,
    pub parent_id: Option<Uuid>,
    pub subtasks: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone)]
pub struct UpdateTodoInput {
    pub id: Uuid,
    pub new_title: Option<String>,
    pub new_description: Option<String>,
    pub new_priority: Option<Priority>,
    pub new_status: Option<Status>,
    pub new_due_date: Option<DateTime<Utc>>,
    pub new_recurrence: Option<Recurrence>,
    pub new_tags: Option<Vec<String>>,
    pub new_parent_id: Option<Uuid>,
    pub new_subtasks: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone)]
pub struct SearchTodoInput {
    pub id: Option<String>,
    pub title: Option<String>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub due_date: Option<DateTime<Utc>>,
    pub recurrence: Option<Recurrence>,
    pub tags: Option<Vec<String>>,
    pub parent_id: Option<Uuid>,
}
