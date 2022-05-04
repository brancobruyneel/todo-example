use serde::{Deserialize, Serialize};

use crate::schema::tasks;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTask {
    pub title: String,
}
