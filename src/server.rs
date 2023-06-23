use actix_web::{ App, HttpServer };
use std::io::{ Result };

use crate::routes;

pub async fn server() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?.run().await
}