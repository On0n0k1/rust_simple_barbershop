use crate::{dao::Service, error::Error, Dao};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    description: String,
    price: u32,
}

// /// #[post("/service/")]
#[post("/service/")]
async fn create_service(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    req: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    log::info!("Create Service called");
    let dao = dao.into_inner();
    let req = req.into_inner();
    let pool: &Pool<Sqlite> = dao.get();
    let mut service: Service = Service::new(req.description, req.price);
    service.create(pool).await?;
    Ok(HttpResponse::Ok().json(service))
}
