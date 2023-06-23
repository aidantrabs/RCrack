use actix_web::{ web, get, post, HttpResponse, Responder };
use tera::{ Tera, Context };

use crate::data::FormData;
use crate::rcrack::rcrack_sha1;

#[get("/")]
async fn form(tera: web::Data<Tera>) -> impl Responder {
    let body = tera.render("form.html.tera", &Context::new()).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("/crack")]
async fn crack(form_data: web::Form<FormData>) -> impl Responder {
    let decoded = rcrack_sha1(&form_data.hash);
    format!("Decoded result: {}", decoded)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(form);
    cfg.service(crack);
}