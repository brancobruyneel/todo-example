use actix_web::{web, get, post, HttpResponse, Error};

use crate::{DbPool, db, models};


#[get("/task")]
async fn get_tasks(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let tasks = web::block(move || {
        let conn = pool.get()?;
        db::list_all_tasks(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(tasks) = tasks {
        Ok(HttpResponse::Ok().json(tasks))
    } else {
        let res = HttpResponse::NotFound().body("No tasks found!".to_string());
        Ok(res)
    }
}

#[post("/task")]
async fn add_task(
    pool: web::Data<DbPool>,
    form: web::Json<models::InputTask>,
) -> Result<HttpResponse, Error> {
    let task = web::block(move || {
        let conn = pool.get()?;
        db::insert_new_task(&form.title, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(task))
}
