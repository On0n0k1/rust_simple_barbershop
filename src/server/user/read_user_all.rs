use crate::{dao::User, error::Error, Dao};
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// /// #[post("/customer/")]
#[get("/user/")]
async fn read_user_all(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
) -> Result<HttpResponse, Error> {
    log::info!("Read Users called");
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let users = User::read_all(pool).await?;
    Ok(HttpResponse::Ok().json(users))
}
