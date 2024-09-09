use actix_web::{web, HttpResponse};

pub fn health_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health")
            .route(web::get().to(|| async { HttpResponse::Ok().body("health") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
