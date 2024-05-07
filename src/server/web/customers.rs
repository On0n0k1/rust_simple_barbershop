use crate::{dao::Customer, Dao};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[get("/customers/")]
async fn customers(
    tera: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("Web Customer called");
    let pool: &Pool<Sqlite> = dao.get();
    let customers: Vec<Customer> = Customer::read_all(pool).await?;

    let mut context = tera::Context::new();
    context.insert("customers", &customers);
    let rendered = tera.render("customer.html", &context).map_err(|e| {
        log::error!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
