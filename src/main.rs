use actix_web::web::{self, ServiceConfig};
use dao::Dao;
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::Arc;
use tera::Tera;

mod dao;
mod error;
mod server;

const TEMPLATES_PATH: &str = "templates/**/*";

// #[get("/")]
// async fn index(
//     tera: web::Data<Arc<tera::Tera>>,
//     _: web::Data<Arc<Dao>>,
// ) -> Result<impl Responder, actix_web::Error> {
//     log::info!("index called");
//     // NamedFile::open_async("templates/index.html").await
//     let context = tera::Context::new();
//     // context.insert("potato", "potatto");
//     let rendered = tera.render("index.html", &context).map_err(|e| {
//         println!("{e:?}");
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(rendered))
// }

// #[get("/menu/")]
// async fn menu(
//     tera: web::Data<Arc<tera::Tera>>,
//     _: web::Data<Arc<Dao>>,
// ) -> Result<impl Responder, actix_web::Error> {
//     log::info!("Menu called");
//     let context = tera::Context::new();
//     let rendered = tera.render("menu.html", &context).map_err(|e| {
//         println!("{e:?}");
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(rendered))
// }

// #[get("/schedule/")]
// async fn schedule(
//     tera: web::Data<Arc<tera::Tera>>,
//     _: web::Data<Arc<Dao>>,
// ) -> Result<impl Responder, actix_web::Error> {
//     log::info!("Schedule called");
//     let context = tera::Context::new();
//     let rendered = tera.render("schedule.html", &context).map_err(|e| {
//         log::error!("{e:?}");
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(rendered))
// }

// #[get("/static/{file}")]
// async fn static_file(
//     _: web::Data<Arc<tera::Tera>>,
//     _: web::Data<Arc<Dao>>,
//     file: web::Path<String>,
// ) -> impl Responder {
//     let file: String = file.into_inner();
//     log::info!("Retrieving file {file}");
//     let path: &String = &format!("static/{file}");
//     match NamedFile::open_async(path).await {
//         Ok(value) => Ok(value),
//         Err(err) => {
//             log::error!("Error when retrieving file: {err:?}");
//             Err(err)
//         }
//     }
// }

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // #[actix_web::main]
    // async fn main() -> std::io::Result<()> {
    let tera: Tera = match Tera::new(TEMPLATES_PATH) {
        Ok(t) => t,
        Err(err) => {
            println!("Error Parsing Tera: {err} ");
            std::process::exit(1);
        }
    };

    let tera: Arc<Tera> = Arc::new(tera);
    let dao: Dao = Dao::new().await.unwrap();
    let dao: Arc<Dao> = Arc::new(dao);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(tera))
            .app_data(web::Data::new(dao.clone()))
            .service(server::web::sign_page::sign_page)
            .service(server::web::menu::menu)
            .service(server::web::schedules::schedules)
            .service(server::web::customers::customers)
            .service(server::web::services::services)
            .service(server::web::users::users)
            .service(server::static_file::static_file)
            .service(server::signin::signin)
            .service(server::customer::create_customer::create_customer)
            .service(server::customer::read_customer::read_customer)
            .service(server::customer::read_customer_all::read_customer_all)
            .service(server::schedule::create_schedule::create_schedule)
            .service(server::schedule::read_schedule::read_schedule)
            .service(server::schedule::read_schedule_all::read_schedule_all)
            .service(server::service::create_service::create_service)
            .service(server::service::read_service::read_service)
            .service(server::service::read_service_all::read_service_all)
            .service(server::user::create_user::create_user)
            .service(server::user::read_user::read_user)
            .service(server::user::read_user_all::read_user_all)
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 100))
            .app_data(web::PayloadConfig::default().limit(1024 * 1024 * 100));
    };
    // Ok(())
    Ok(config.into())
}
