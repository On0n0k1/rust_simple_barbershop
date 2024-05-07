use crate::{dao::Schedule, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[get("/schedule/")]
#[get("/schedule/")]
async fn read_schedule_all(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<HttpResponse, Error> {
    log::info!("Read Schedule All called");
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let schedules: Vec<Schedule> = Schedule::read_all(pool).await?;
    Ok(HttpResponse::Ok().json(schedules))
}
