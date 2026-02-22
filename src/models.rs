use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginUser {
    pub id: Option<Uuid>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateAcccount {
    pub firstName:String,
    pub lastName: String,
    pub email:String,
    pub password: String
}
