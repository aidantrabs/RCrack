use std::io::{ Result };

mod routes;
mod server;

#[actix_web::main]
async fn main() -> Result<()> {
    server::server().await
}