use actix_web::{App, HttpServer, web, middleware::Logger};

mod database_config;
mod handlers;
mod utils;
mod models;
mod routes;
mod middleware;

use middleware::GeoIpMiddleware;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = database_config::connection_setting().await;


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(GeoIpMiddleware)
            .wrap(Logger::default())
            .configure(routes::registration_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
