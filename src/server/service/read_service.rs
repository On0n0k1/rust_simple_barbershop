use crate::{dao::Service, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[post("/customer/")]
#[get("/service/{id}")]
async fn read_service(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    log::info!("Read Service called");
    let id: u32 = path.into_inner();
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let service: Service = Service::read(id, pool).await?;
    Ok(HttpResponse::Ok().json(service))
}
