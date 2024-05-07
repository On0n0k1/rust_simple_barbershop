use crate::{dao::Schedule, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[get("/schedule/{id}")]
#[get("/schedule/{id}")]
async fn read_schedule(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    log::info!("Read Schedule called");
    let id = path.into_inner();
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let schedule: Schedule = Schedule::read(id, pool).await?;
    Ok(HttpResponse::Ok().json(schedule))
}
