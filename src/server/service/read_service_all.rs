use crate::{dao::Service, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[get("/service/")]
#[get("/service/")]
async fn read_service_all(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<HttpResponse, Error> {
    log::info!("Read Services called");
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let service: Vec<Service> = Service::read_all(pool).await?;
    Ok(HttpResponse::Ok().json(service))
}
