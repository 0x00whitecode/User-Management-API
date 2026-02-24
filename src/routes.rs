use actix_web::web;
use crate::handlers::{register_user, signin};



pub fn registration_routes(cfg: &mut web::ServiceConfig) {
    cfg.service (
        web::scope("/auth")
            .route("/register", web::post().to(register_user))
            .route("/signin", web::post().to(signin)),
    );
}