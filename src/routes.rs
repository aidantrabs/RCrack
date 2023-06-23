use actix_web::{ HttpResponse, Responder }

#[get("/")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Working");
}
