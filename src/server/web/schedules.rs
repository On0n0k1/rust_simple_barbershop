use crate::{
    dao::{Customer, Schedule, Service},
    Dao,
};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[get("/schedules/")]
async fn schedules(
    tera: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("Web Schedule called");
    let pool: &Pool<Sqlite> = dao.get();
    let services: Vec<Service> = Service::read_all(pool).await?;
    let schedules: Vec<Schedule> = Schedule::read_all(pool).await?;
    let customers: Vec<Customer> = Customer::read_all(pool).await?;

    let mut context = tera::Context::new();
    context.insert("services", &services);
    context.insert("schedules", &schedules);
    context.insert("customers", &customers);
    let rendered = tera.render("schedule.html", &context).map_err(|e| {
        log::error!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
