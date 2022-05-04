use diesel::prelude::*;
use uuid::Uuid;

use crate::models::Task;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn list_all_tasks(conn: &SqliteConnection) -> Result<Option<Vec<Task>>, DbError> {
    use crate::schema::tasks::dsl::*;

    Ok(tasks.load::<Task>(conn).optional()?)
}

pub fn insert_new_task(t: &str, conn: &SqliteConnection) -> Result<Task, DbError> {
    use crate::schema::tasks::dsl::*;

    let new_task = Task {
        id: Uuid::new_v4().to_string(),
        title: t.to_owned(),
        completed: false,
    };

    diesel::insert_into(tasks).values(&new_task).execute(conn)?;

    Ok(new_task)
}
