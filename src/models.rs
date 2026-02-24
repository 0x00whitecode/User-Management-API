#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize)]
pub struct RegisterUser {
    pub id: Option<Uuid>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub is_email_verified: bool,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub otp: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub ip: Option<String>,
}


#[derive(Deserialize)]
pub struct ClientLocation {
    latitude: f64,
    longitude: f64,
}
