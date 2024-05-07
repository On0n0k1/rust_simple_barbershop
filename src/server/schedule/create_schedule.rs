use crate::{dao::Schedule, error::Error, Dao};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    customer_id: u32,
    service_id: u32,
    price: u32,
    date: String,
    hour: u32,
    details: String,
}

// /// #[post("/customer/")]
#[post("/schedule/")]
async fn create_schedule(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    req: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    log::info!("Create Schedule called");
    let dao = dao.into_inner();
    let req = req.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let tx = pool.begin().await.or_else(Error::sqlx)?;
    let (schedule, tx) = Schedule::new(
        req.customer_id,
        req.service_id,
        req.price,
        req.date,
        req.hour,
        req.details,
        tx,
    )
    .await?;

    // let mut customer = Customer::new(
    //     req.name,
    //     req.gender,
    //     req.birth_date,
    //     req.phone,
    //     req.email,
    //     req.rg,
    //     req.address,
    //     req.cep,
    // );
    tx.commit().await.or_else(Error::sqlx)?;
    // schedule.create(pool).await?;
    Ok(HttpResponse::Ok().json(schedule))
}
