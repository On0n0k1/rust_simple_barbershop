use crate::Dao;
use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self},
    Responder,
};
use std::sync::Arc;

#[get("/static/{file}")]
async fn static_file(
    _: web::Data<Arc<tera::Tera>>,
    _: web::Data<Arc<Dao>>,
    file: web::Path<String>,
) -> impl Responder {
    let file: String = file.into_inner();
    log::info!("Retrieving file {file}");
    let path: &String = &format!("static/{file}");
    match NamedFile::open_async(path).await {
        Ok(value) => Ok(value),
        Err(err) => {
            log::error!("Error when retrieving file: {err:?}");
            Err(err)
        }
    }
}
