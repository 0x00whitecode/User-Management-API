use actix_web::{App, HttpServer, web};

mod database_config;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = database_config::connection_setting().await;


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
