use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger::{Env};
use actix_web::middleware::Logger;
use rest_api_app::interfaces::web::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(configure_routes)
            .route("/", web::get().to(||async {
                HttpResponse::Ok().body("connected to server.")
            }))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
