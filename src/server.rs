use actix_web::{ App, HttpServer, web::{self, Data} };
use std::io::{ Result };
use tera::Tera;

use actix_files as fs;
use crate::routes;

pub async fn server() -> Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .service(fs::Files::new("/static", "static"))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}