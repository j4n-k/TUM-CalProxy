use crate::error;
use actix_web::http::Method;
use actix_web::{web, Responder, Scope};

pub fn service() -> Scope {
    Scope::new("/api/health")
        .route("", web::get().to(handler))
        .default_service(
            web::route().to(|| async { error::MethodNotAvailable::new(&[&Method::GET]) }),
        )
}

async fn handler() -> impl Responder {
    "OK :)"
}
