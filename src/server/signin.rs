use crate::error::Error;
use crate::{dao::User, Dao};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[post("/signin/")]
async fn signin(
    _: web::Data<Arc<tera::Tera>>,
    dao: web::Data<Arc<Dao>>,
    req: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    log::info!("Signin called");
    let req: Request = req.into_inner();
    let dao = dao.into_inner();
    let pool: &Pool<Sqlite> = dao.get();

    let user: User = match User::read_name(&req.username, pool).await {
        Ok(user) => user,
        Err(err) => {
            let msg = format!("{err:?}");
            return Ok(HttpResponse::Unauthorized().body(msg));
        }
    };

    let is_correct = user.check_password(&req.password);
    if is_correct {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
