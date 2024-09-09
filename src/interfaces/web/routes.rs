use crate::interfaces::controllers::user_controller::get_users;
use actix_web::{web, HttpResponse};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health")
            .route(web::get().to(|| async { HttpResponse::Ok().body("health") })),
    )
    .service(web::scope("/users").service(get_users));
}
