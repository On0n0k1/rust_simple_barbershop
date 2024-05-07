use crate::{dao::User, error::Error, Dao};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub name: String,
    pub password: String,
    pub access: u32,
    pub gender: String,
    pub date_of_birth: String,
    pub phone: String,
    pub email: String,
    pub rg: String,
}

// /// #[post("/user/")]
#[post("/user/")]
async fn create_user(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    req: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    log::info!("Create User called");
    let req = req.into_inner();
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();
    let mut user = User::new(
        req.name,
        req.password,
        0,
        req.gender,
        req.date_of_birth,
        req.phone,
        req.email,
        req.rg,
    );
    user.create(pool).await?;
    Ok(HttpResponse::Ok().json(user))
}
