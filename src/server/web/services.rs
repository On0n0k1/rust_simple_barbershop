use crate::{dao::Service, Dao};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[get("/services/")]
async fn services(
    tera: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("Web Services called");
    let pool: &Pool<Sqlite> = dao.get();
    let services: Vec<Service> = Service::read_all(pool).await?;

    let mut context = tera::Context::new();
    context.insert("services", &services);
    let rendered = tera.render("services.html", &context).map_err(|e| {
        log::error!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
