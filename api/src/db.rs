use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{Task, TaskView};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn list_all_tasks(conn: &SqliteConnection) -> Result<Option<Vec<TaskView>>, DbError> {
    use crate::schema::tasks::dsl::*;

    let all_tasks = tasks.load::<Task>(conn).optional()?;

    if let Some(all) = all_tasks {
        let all_views: Vec<TaskView> = all.into_iter().map(|task| task.into()).collect();
        Ok(Some(all_views))
    } else {
        panic!("Something went wrong with the database!");
    }
}

pub fn insert_new_task(t: &str, c: bool, conn: &SqliteConnection) -> Result<TaskView, DbError> {
    use crate::schema::tasks::dsl::*;

    let new_task = Task {
        id: Uuid::new_v4().to_string(),
        title: t.to_owned(),
        completed: c as i32,
    };

    let result = diesel::insert_into(tasks).values(&new_task).execute(conn);

    match result {
        Ok(_) => Ok(TaskView {
            id: new_task.id,
            title: new_task.title,
            completed: new_task.completed != 0,
        }),
        Err(_) => panic!("Something went wrong with the database!"),
    }
}
