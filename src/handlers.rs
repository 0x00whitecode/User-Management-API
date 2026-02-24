use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;


use crate::models::{LoginUser, RegisterUser};
use crate::utils::{hash_password, verify_password, create_jwt};
use crate::middleware::get_location_from_request;

pub async fn register_user(pool: web::Data<PgPool>, data: web::Json<RegisterUser>) -> HttpResponse {
    let id = Uuid::new_v4();
    // let hash the password to store on the database for security purpose
    let passwordhashing = hash_password(&data.password);


    let res = sqlx::query!(
        "INSERT INTO register_user (id, email, first_name, last_name, password) VALUES ($1, $2, $3, $4, $5)",
        id,
        data.email,
        data.first_name,
        data.last_name,
        passwordhashing
    )
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}


pub async fn signin(pool: web::Data<PgPool>, req: HttpRequest, data: web::Json<LoginUser>) -> HttpResponse {
    // Extract geolocation from middleware
    let location = get_location_from_request(&req);

    // Extract user agent info
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let record = sqlx::query!(
        "SELECT id, password FROM register_user WHERE email = $1",
        data.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    let record = match record {
        Ok(Some(r)) => r,
        _ => {
            // Log failed login attempt (user not found)
            let login_id = Uuid::new_v4();
            let ip_str = location.as_ref().and_then(|l| l.ip.as_ref()).map(|s| s.as_str()).unwrap_or("0.0.0.0");
            let lat_str = location.as_ref().map(|l| l.latitude.to_string());
            let lon_str = location.as_ref().map(|l| l.longitude.to_string());

            let _ = sqlx::query(
                "INSERT INTO login_user (id, user_id, device, os, browser, ip_address, latitude, longitude, signin_success, failure_reason) 
                 VALUES ($1, NULL, $2, NULL, NULL, $3::inet, $4, $5, false, $6)"
            )
            .bind(login_id)
            .bind(user_agent.as_deref())
            .bind(ip_str)
            .bind(lat_str)
            .bind(lon_str)
            .bind("User not found")
            .execute(pool.get_ref())
            .await;

            return HttpResponse::Unauthorized().finish();
        }
    };

    if verify_password(&record.password, &data.password) {
        // Log successful login
        let login_id = Uuid::new_v4();
        let ip_str = location.as_ref().and_then(|l| l.ip.as_ref()).map(|s| s.as_str()).unwrap_or("0.0.0.0");
        let lat_str = location.as_ref().map(|l| l.latitude.to_string());
        let lon_str = location.as_ref().map(|l| l.longitude.to_string());

        let _ = sqlx::query(
            "INSERT INTO login_user (id, user_id, device, os, browser, ip_address, latitude, longitude, signin_success, failure_reason) 
             VALUES ($1, $2, $3, NULL, NULL, $4::inet, $5, $6, true, NULL)"
        )
        .bind(login_id)
        .bind(record.id)
        .bind(user_agent.as_deref())
        .bind(ip_str)
        .bind(lat_str)
        .bind(lon_str)
        .execute(pool.get_ref())
        .await;

        let token = create_jwt(&record.id.to_string());
        HttpResponse::Ok().json(serde_json::json!({ "token": token }))
    } else {
        // Log failed password attempt
        let login_id = Uuid::new_v4();
        let ip_str = location.as_ref().and_then(|l| l.ip.as_ref()).map(|s| s.as_str()).unwrap_or("0.0.0.0");
        let lat_str = location.as_ref().map(|l| l.latitude.to_string());
        let lon_str = location.as_ref().map(|l| l.longitude.to_string());

        let _ = sqlx::query(
            "INSERT INTO login_user (id, user_id, device, os, browser, ip_address, latitude, longitude, signin_success, failure_reason) 
             VALUES ($1, $2, $3, NULL, NULL, $4::inet, $5, $6, false, $7)"
        )
        .bind(login_id)
        .bind(record.id)
        .bind(user_agent.as_deref())
        .bind(ip_str)
        .bind(lat_str)
        .bind(lon_str)
        .bind("Invalid password")
        .execute(pool.get_ref())
        .await;

        HttpResponse::Unauthorized().finish()
    }
}


// opt

