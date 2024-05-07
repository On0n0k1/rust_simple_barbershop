use crate::{dao::User, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[post("/customer/")]
#[get("/user/{id}")]
async fn read_user(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    log::info!("Read User called");
    let id = path.into_inner();
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let user = User::read(&id, pool).await?.output();
    Ok(HttpResponse::Ok().json(user))
}
