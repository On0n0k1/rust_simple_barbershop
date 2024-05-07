use crate::Dao;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/")]
async fn sign_page(
    tera: web::Data<Arc<tera::Tera>>,
    _: web::Data<Arc<Dao>>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("index called");
    // NamedFile::open_async("templates/index.html").await
    let context = tera::Context::new();
    // context.insert("potato", "potatto");
    let rendered = tera.render("index.html", &context).map_err(|e| {
        println!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
