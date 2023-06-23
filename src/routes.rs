use actix_web::{ get, HttpResponse, Responder, web };

#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Working")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
}