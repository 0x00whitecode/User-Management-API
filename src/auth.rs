use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use crate::models::Claims;
use std::env;


pub fn create_jwt(user_id: &str) -> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET is needed here");

    let claims = Claims{
        sub: user_id.to_string(),
        exp: (UTC::now() + Duration::hour(24).timestamp()) as usize
    }

    encode (
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
    .unwrap()
}


pub fn verify_jwt(token: &str) -> Claims {
    let secret = env::var("JWT_SECRET").unwrap();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    )
    .unwrap()
    .claims
}


pub async fn forgot_password(pool: web::Data<PgPool>,data: web::Json<ForgotPasswordRequest>,) -> HttpResponse {
    let otp = generate_otp();
    let otp_hash = hash_otp(&otp);
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);
    let id = Uuid::new_v4();

    let res = sqlx::query!(
        "INSERT INTO password_resets (id, email, otp_hash, expires_at)
         VALUES ($1, $2, $3, $4)",
        id,
        data.email,
        otp_hash,
        expires_at
    )
    .execute(pool.get_ref())
    .await;

    if res.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    // 🔔 SEND OTP (email / sms)
    println!("OTP for {} is {}", data.email, otp);

    HttpResponse::Ok().json({
        "message": "OTP sent"
    })
}

pub async fn verify_otp_handler(pool: web::Data<PgPool>, data: web::Json<VerifyOtpRequest>,) -> HttpResponse {
    let record = sqlx::query!(
        "SELECT id, otp_hash, expires_at, used
         FROM password_resets
         WHERE email = $1
         ORDER BY created_at DESC
         LIMIT 1",
        data.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    match record {
        Ok(Some(r)) => {
            if r.used {
                return HttpResponse::BadRequest().json("OTP already used");
            }

            if chrono::Utc::now() > r.expires_at {
                return HttpResponse::BadRequest().json("OTP expired");
            }

            if verify_otp(&r.otp_hash, &data.otp) {
                HttpResponse::Ok().json("OTP verified")
            } else {
                HttpResponse::Unauthorized().json("Invalid OTP")
            }
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}


pub async fn reset_password(pool: web::Data<PgPool>,data: web::Json<ResetPasswordRequest>,) -> HttpResponse {
    let record = sqlx::query!(
        "SELECT id, otp_hash, expires_at, used
         FROM password_resets
         WHERE email = $1
         ORDER BY created_at DESC
         LIMIT 1",
        data.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    let record = match record {
        Ok(Some(r)) => r,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    if record.used || chrono::Utc::now() > record.expires_at {
        return HttpResponse::BadRequest().finish();
    }

    if !verify_otp(&record.otp_hash, &data.otp) {
        return HttpResponse::Unauthorized().finish();
    }

    let new_hash = hash_password(&data.new_password);

    let _ = sqlx::query!(
        "UPDATE users SET password_hash = $1 WHERE email = $2",
        new_hash,
        data.email
    )
    .execute(pool.get_ref())
    .await;

    let _ = sqlx::query!(
        "UPDATE password_resets SET used = TRUE WHERE id = $1",
        record.id
    )
    .execute(pool.get_ref())
    .await;

    HttpResponse::Ok().json("Password reset successful")
}
