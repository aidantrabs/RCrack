use std::io::{ Result };
use std::env::{ set_var };
use crate::server::{ server };

mod server;
mod routes;
mod rcrack;
mod data;

#[actix_web::main]
async fn main() -> Result<()> {
    set_var("RUST_LOG", "debug");
    server().await
}