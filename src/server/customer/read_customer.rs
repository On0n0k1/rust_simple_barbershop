use crate::{dao::Customer, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[get("/customer/{id}")]
#[get("/customer/{id}")]
async fn read_customer(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    log::info!("Read Customer called");
    let id = path.into_inner();
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let customer = Customer::read(&id, pool).await?;
    Ok(HttpResponse::Ok().json(customer))
}
