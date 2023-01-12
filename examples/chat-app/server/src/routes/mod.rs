use actix_web::{web::ServiceConfig, get};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index);
}
