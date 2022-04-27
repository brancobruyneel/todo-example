use serde::{Deserialize, Serialize};

use crate::schema::tasks;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: i32,
}

impl From<Task> for TaskView {
    fn from(val: Task) -> Self {
        TaskView {
            id: val.id,
            title: val.title,
            completed: val.completed != 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskView {
    pub id: String,
    pub title: String,
    pub completed: bool,
}
