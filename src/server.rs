use actix_web::{ App, HttpServer };
use std::io::{ Result };
use crate::routes;

pub async fn server() -> Result<()> {
    let app = move || {
        App::new()
            .service(routes::test)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}