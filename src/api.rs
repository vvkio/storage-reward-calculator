use actix_web::{web, HttpResponse, Responder};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/ping").route(web::get().to(ping)));
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
