use crate::{dao::Customer, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[post("/customer/")]
#[get("/customer/")]
async fn read_customer_all(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<HttpResponse, Error> {
    log::info!("Read Customer called");
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let customers = Customer::read_all(pool).await?;
    Ok(HttpResponse::Ok().json(customers))
}
