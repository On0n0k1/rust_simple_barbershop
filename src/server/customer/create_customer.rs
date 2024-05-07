use crate::{dao::Customer, error::Error, Dao};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub gender: String,
    pub birth_date: String,
    pub phone: String,
    pub email: String,
    pub rg: String,
    pub address: String,
    pub cep: String,
}

// /// #[post("/customer/")]
#[post("/customer/")]
async fn create_customer(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    req: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    log::info!("Create Customer called");
    let dao = dao.into_inner();
    let req = req.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let mut customer = Customer::new(
        req.name,
        req.gender,
        req.birth_date,
        req.phone,
        req.email,
        req.rg,
        req.address,
        req.cep,
    );
    customer.create(pool).await?;
    Ok(HttpResponse::Ok().json(customer))
}
