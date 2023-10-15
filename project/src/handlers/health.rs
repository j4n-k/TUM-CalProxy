use actix_web::{Responder, Scope, web};
use actix_web::http::Method;
use crate::error;

pub fn service() -> Scope {
    Scope::new("/api/health")
        .route("", web::get().to(handler))
        .default_service(web::route().to(|| async {
            error::MethodNotAvailable::new(&[&Method::GET])
        }))
}

async fn handler() -> impl Responder {
    "OK :)"
}