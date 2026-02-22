use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;



pub async fn connection_setting() -> PgPool {
    dotenv().ok();

    let environment = env::var("ENV").unwrap_or_else(|_| "local".to_string());

    let database_url = match environment.as_str() {
        "prod" => env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set"),
        _ => env::var("LOCAL_DATABASE_URL").expect("LOCAL_DATABASE_URL must be set"),
    };

    println!("Connecting to database in {} environment", environment);

    PgPool::connect(&database_url).await.expect("Failed to connect to the database; check your DATABASE_URL settings")

}
