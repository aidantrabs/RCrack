use std::io::{ Result };

mod server;

#[actix_web::main]
async fn main() -> Result<()> {
    server::server().await
}