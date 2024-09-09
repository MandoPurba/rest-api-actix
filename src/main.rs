use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger::Env;
use rest_api_app::configs::config::Config;
use rest_api_app::infrastructures::database::Database;
use rest_api_app::interfaces::web::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    let state = Database::new(config.database_url).await;

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(state.pg_pool.clone()))
            .configure(configure_routes)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("connected to server.") }),
            )
    })
    .bind(("0.0.0.0", config.app_port))?
    .run()
    .await
}
