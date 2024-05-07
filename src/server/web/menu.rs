use crate::Dao;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/menu/")]
async fn menu(
    tera: web::Data<Arc<tera::Tera>>,
    _: web::Data<Arc<Dao>>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("Menu called");
    let context = tera::Context::new();
    let rendered = tera.render("menu.html", &context).map_err(|e| {
        println!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
