use crate::interfaces::web::health::health_route;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    health_route(cfg);
}
