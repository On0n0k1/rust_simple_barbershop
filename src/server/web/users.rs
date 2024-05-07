use crate::{dao::User, Dao};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[get("/users/")]
async fn users(
    tera: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<impl Responder, actix_web::Error> {
    log::info!("Web Users called");
    let pool: &Pool<Sqlite> = dao.get();
    let users: Vec<User> = User::read_all(pool).await?;

    let mut context = tera::Context::new();
    context.insert("users", &users);
    let rendered = tera.render("user.html", &context).map_err(|e| {
        log::error!("{e:?}");
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
